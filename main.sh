#! /bin/bash

TIMEFORMAT='It took %R seconds.'
time {
./target/release/leef
}
