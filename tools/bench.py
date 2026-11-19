#!/usr/bin/env python3
import datetime
import os
from pathlib import Path
import re
import shlex
import shutil
import subprocess
import sys
import time
import unittest

ROOT = Path(__file__).resolve().parent.parent
SOURCE = ROOT / 'benches/hi_lite.rs'
TIERS = ('edit', 'change', 'scenario', 'full')


def registry():
    source = SOURCE.read_text()
    pairs = re.findall(r'\((hi_lite_\w+), (hi_lite_\w+_cold), \d+\)', source)
    entries = [('selector', 'edit', 'driver', 'selection and preflight contracts')]
    entries += [(name, 'change' if name in ('hi_lite_rust', 'hi_lite_unified_diff_warm') else 'full',
                 'highlight' if name != 'hi_lite_unified_diff_warm' else 'diff', 'fixture throughput and allocations')
                for name in ['hi_lite_rust', 'hi_lite_unified_diff_warm']]
    entries += [(name, 'scenario', 'diff' if 'diff' in name else 'highlight', 'fixture throughput and allocations')
                for name in re.findall(r'fn (hi_lite_(?:highlight_\w+|unified_diff_\w+))\(bencher:', source)
                if name != 'hi_lite_unified_diff_warm']
    entries += [(name, 'full', 'highlight', 'individual language throughput and allocations')
                for pair in pairs for name in pair if name != 'hi_lite_rust']
    return entries


def select(entries, tier, names, areas, skips):
    if tier not in TIERS:
        raise ValueError('unknown tier: ' + tier)
    known_names = {e[0] for e in entries}
    known_areas = {e[2] for e in entries}
    for label, values, known in [('workload', names, known_names), ('area', areas, known_areas), ('skip', skips, known_names)]:
        for value in values:
            if value not in known:
                raise ValueError('unknown ' + label + ': ' + value)
    chosen = [e for e in entries if TIERS.index(e[1]) <= TIERS.index(tier)
              and (not names or e[0] in names) and (not areas or e[2] in areas) and e[0] not in skips]
    if not chosen:
        raise ValueError('selection matches nothing')
    return chosen


def words(key):
    return os.environ.get(key, '').replace(',', ' ').split()


def preflight(chosen):
    problems = []
    for tool in ('cargo', 'rustc'):
        if not shutil.which(tool):
            problems.append(f'{tool} missing; install the pinned Rust toolchain')
    for tool in ('cargo', 'rustc'):
        if shutil.which(tool):
            result = subprocess.run([tool, '--version'], cwd=ROOT, capture_output=True, text=True)
            if result.returncode:
                problems.append(f'{tool} cannot run; install the pinned Rust toolchain')
    if any(e[2] != 'driver' for e in chosen):
        for rel in re.findall(r'include_(?:str|bytes)!\("\.\./([^\"]+)"\)', SOURCE.read_text()):
            if not (ROOT / rel).is_file():
                problems.append(f'{rel} missing; restore the checked-in fixture')
        if shutil.disk_usage(ROOT).free < 512 * 1024 * 1024:
            problems.append('less than 512 MiB free; free disk for Cargo builds')
    if problems:
        raise ValueError('; '.join(problems) + '; needed by ' + ', '.join(e[0] for e in chosen))


def command(entry):
    if entry[0] == 'selector':
        return [sys.executable, str(ROOT / 'tools/bench.py'), 'self-test']
    return ['cargo', 'bench', '--locked', '--bench', 'hi_lite', '--', 'hi_lite::' + entry[0], '--exact', '--sample-count', '10', '--min-time', '0.1', '--max-time', '0.2']


class SelectionTests(unittest.TestCase):
    def test_selection_and_preflight(self):
        entries = registry()
        self.assertEqual([e[0] for e in select(entries, 'edit', [], [], [])], ['selector'])
        self.assertEqual([e[0] for e in select(entries, 'change', ['hi_lite_rust'], ['highlight'], [])], ['hi_lite_rust'])
        for args in [('change', ['unknown'], [], []), ('edit', [], ['diff'], []), ('edit', [], [], ['selector'])]:
            with self.assertRaises(ValueError):
                select(entries, *args)
        with unittest.mock.patch('shutil.which', return_value=None):
            with self.assertRaises(ValueError):
                preflight(select(entries, 'edit', [], [], []))


def main():
    action = sys.argv[1]
    if action == 'self-test':
        import unittest.mock
        unittest.main(argv=[sys.argv[0]])
        return
    entries = registry()
    if action == 'list':
        for name, tier, area, purpose in entries:
            print(f'{name}\t{tier}\t{area}\t{purpose}')
        return
    try:
        chosen = select(entries, os.environ.get('TIER', 'edit').lower(), words('NAME'), words('AREA'), words('SKIP'))
        preflight(chosen)
    except ValueError as error:
        print('bench preflight: ' + str(error), file=sys.stderr)
        sys.exit(2)
    selected = {e[0] for e in chosen}
    for entry in entries:
        if entry[0] not in selected:
            print(f'excluded {entry[0]}: tier or selector')
    print('Host readiness: not assessed; timing is exploratory, not a performance verdict.')
    if action == 'check':
        print('preflight passed: ' + ', '.join(e[0] for e in chosen))
        return
    dry = os.environ.get('DRY_RUN', '') == '1'
    for entry in chosen:
        print(' '.join(shlex.quote(arg) for arg in command(entry)))
    if dry:
        return
    output = ROOT / '.work' / 'bench' / datetime.datetime.now().strftime('%Y%m%d-%H%M%S-%f')
    output.mkdir(parents=True)
    failed = False
    with (output / 'summary.txt').open('w') as summary:
        summary.write('Timing validity: host not assessed; no performance verdict or budget.\n')
        for args in (['git', 'rev-parse', 'HEAD'], ['git', 'status', '--short'], ['rustc', '--version']):
            result = subprocess.run(args, cwd=ROOT, capture_output=True, text=True)
            summary.write(' '.join(args) + ': ' + (result.stdout.strip() or '(clean)') + '\n')
        for entry in entries:
            if entry[0] not in selected:
                summary.write(f'excluded {entry[0]}: tier or selector\n')
        for entry in chosen:
            start = time.monotonic()
            with (output / (entry[0] + '.log')).open('w') as log:
                result = subprocess.run(command(entry), cwd=ROOT, stdout=log, stderr=subprocess.STDOUT)
            missing = entry[0] != 'selector' and entry[0] not in (output / (entry[0] + '.log')).read_text()
            line = f'{entry[0]}: exit {result.returncode}, {time.monotonic() - start:.1f}s'
            if missing:
                line += ', no benchmark result'
                failed = True
            print(line, flush=True)
            summary.write('command: ' + shlex.join(command(entry)) + '\n' + line + '\n')
            failed |= result.returncode != 0
    print(f'logs: {output}')
    sys.exit(int(failed))


if __name__ == '__main__':
    main()
