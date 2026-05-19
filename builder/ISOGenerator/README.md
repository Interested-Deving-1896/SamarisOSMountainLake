# Samaris OS ISOGenerator

`ISOGenerator` is the canonical release build pipeline for Samaris OS Alpha One RC.

It builds a universal ISO with two native live systems:

- `x86_64` rootfs, kernel, initrd, and SquashFS
- `aarch64` rootfs, kernel, initrd, and SquashFS

GRUB selects the right live path at boot through `grub_cpu`; BIOS boot uses the x86_64 ISOLINUX path.

Run from any directory:

```bash
/Users/kalo/Desktop/SAMARIS\ OS/run.sh iso --docker
```

The pipeline is checkpointed. Every successful step writes:

- persistent state in the Docker work volume (`/samaris-work/state`)
- visible logs and `.done` markers in `builder/output/checkpoints/`

Useful commands:

```bash
./run.sh status --docker
./run.sh next --docker
./run.sh run 02-ai-assets --docker
./run.sh iso --docker --from 05-packages
./run.sh iso --docker --only 11-electron
./run.sh clean --docker
```

Native Linux builds are also supported when all dependencies are installed:

```bash
./builder/ISOGenerator/generator.sh iso
```
