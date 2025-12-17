#!/bin/bash

cargo watch \
  -x fmt \
  -x check \
  -x 'test -- --no-capture' \
  -x run;
