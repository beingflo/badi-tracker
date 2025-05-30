#!/usr/bin/env bash
die() { echo "$*" 1>&2 ; exit 1; }

echo -e "Deploying badi-tracker to production!"

docker buildx build -t "ghcr.io/beingflo/badi-tracker:0.1.0" . || die "Failed to build docker image"
docker push "ghcr.io/beingflo/badi-tracker:0.1.0" || die "Failed to push docker image"

docker --context arm compose --file compose.prod.yml pull || die "Failed to pull new image"
docker --context arm compose --file compose.prod.yml up -d || die "Failed to bring compose up"