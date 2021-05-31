# Creating a small from-scratch Docker image for a Rust application

This repo shows a way to create a small from-scratch Docker image based
on [this article](https://www.artificialworlds.net/blog/2020/04/22/creating-a-tiny-docker-image-of-a-rust-project/).

In order to work correctly the Rust build fingerprint is deleted explicitly for the binary in the Dockerfile.

## Usage

```bash
docker build -t rust-app .
docker container run --rm -p 8000:8000 --name rust-app rust-app
```