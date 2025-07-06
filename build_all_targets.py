#!/usr/bin/env python3

import os
import sys
import shutil
import subprocess

required_bins = ['cargo', 'zig']

for b in required_bins:
  if not shutil.which(b):
    print(f'[ Fatal Error] required binary "{b}" does not exist. Install and re-run.')
    sys.exit(1)

if not shutil.which('cargo-zigbuild'):
  yn = input(f'Need cargo-zigbuild[.exe] installed, ok to install?').strip().lower()
  if not (yn[:1] in ('y', '1', 't') ):
    print(f'[ Fatal Error] Cannot install cargo-zigbuild, exiting.')
    sys.exit(1)
  subprocess.run([
    'cargo', 'install', '--locked', 'cargo-zigbuild'
  ], check=True)

def find_target_binary(t):
  canidates = [
    os.path.join('target', t, 'release', 'cyber-reachability.exe'),
    os.path.join('target', t, 'release', 'cyber-reachability'),
  ]
  for c in canidates:
    if os.path.exists(c):
      return c
  raise Exception(f'Cannot find a binary for {t}')

targets = [
  'x86_64-pc-windows-gnu',
  'x86_64-unknown-linux-gnu',
  'x86_64-apple-darwin',

  # TODO future R&D stuff
  #'aarch64-pc-windows-gnu',
  #'aarch64-unknown-linux-gnu',
  #'aarch64-apple-darwin',
]

for t in targets:
  print(f'Building for "{t}"')
  subprocess.run([
    'cargo', 'zigbuild', '--release', '--target', f'{t}'
  ], check=True)
  out_bin_path = os.path.abspath(find_target_binary(t))
  print(f'[ Built ] {out_bin_path}')




