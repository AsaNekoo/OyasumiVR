import { exec } from 'child_process';
import { log } from 'console';
import { rm, unlinkSync } from 'fs';
import { mkdirp } from 'mkdirp';
import copy from 'recursive-copy';
import { rimraf } from 'rimraf';

async function main() {
  const release_path = '/tmp/Oyasumi_build/Oyasumi/';
  await rimraf(release_path);
  await mkdirp(release_path);
  await copy('src-core/target/release/OyasumiVR', release_path + 'OyasumiVR', { overwrite: true });
  await copy('src-core/target/release/resources', release_path + 'resources', { overwrite: true });
  await copy('src-core/target/release/cef', release_path + 'resources/sidecars/cef', {
    overwrite: true,
  });
   try {
    await unlinkSync('/tmp/Oyasumi_build/Oyasumi/resources/manifest.vrmanifest');
  } catch {}
  try {
    await unlinkSync('/tmp/Oyasumi_build/Oyasumi/resources/input');
  } catch {}
  console.log("packaging");
  await execPromise('ZSTD_CLEVEL=19 nice -n20 tar -I zstd -cvpf oyasumi-linux.tar.zst Oyasumi/');
  await rimraf("bin/");
  await mkdirp("bin/");
  await copy("/tmp/Oyasumi_build/oyasumi-linux.tar.zst","bin/oyasumi-linux.tar.zst");
  await rimraf("/tmp/Oyasumi_build");

}
const execPromise = (command) => new Promise((resolve, reject) => {
  exec(command,{ cwd: '/tmp/Oyasumi_build/' }, (err, stdout, stderr) => {
    if (err) {
      console.error(err);
      reject(stderr || err);
    } else {
      resolve(stdout);
    }
  });
});
main().catch((e) => {
  throw e;
});
