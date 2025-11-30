import { mkdirp } from 'mkdirp';
import copy from 'recursive-copy';
import { rimraf } from 'rimraf';

async function main() {
  const SourceFile = 'src-core/resources/fonts';
  const TargetDirectory = 'src-core/target/release/resources/fonts';
  await mkdirp(SourceFile);
  await copy(SourceFile, TargetDirectory, { overwrite: true });
  const SourceFile2 = 'src-core/resources/sounds';
  const TargetDirectory2 = 'src-core/target/release/resources/sounds';
  await mkdirp(SourceFile2);
  await copy(SourceFile2, TargetDirectory2, { overwrite: true });
}

main().catch((e) => {
  throw e;
});
