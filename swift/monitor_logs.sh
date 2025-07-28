#!/bin/bash
log stream --predicate 'process == "CoTExampleApp"' | grep -E "(timestamp|📍|🕐|Event [0-9]+:|Found [0-9]+ existing)"