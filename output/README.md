# Build Output

This directory is the designated runtime executable output location for Synthetic Brain Architecture Prototype Alpha 1.0.5.

The GitHub Actions build places the compiled `human-brain` executable in the platform-specific release output under this directory and uploads the same executable as a workflow artifact.

Expected release executable:

- Windows: `output/windows-x86_64/human-brain.exe`
- Linux: `output/linux-x86_64/human-brain`

Generated build products are not committed to Git; use the GitHub Actions artifact from the successful build to retrieve the executable.
