#!/bin/bash
set -eux

bindgen \
  --whitelist-function="^METIS_.*" \
  --whitelist-type=".*_et" \
  --no-prepend-enum-name \
  --default-enum-style=rust \
  --with-derive-{default,eq,hash,ord} \
  --use-core \
  ../metis-src/metis-5.1.0/include/metis.h \
  > src/metis.rs
