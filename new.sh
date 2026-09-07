#!/bin/bash
# Usage: ./new.sh <problem_name> [fastio|simple]

set -e

NAME=${1:?"Problem name required"}
TEMPLATE=${2:-fastio}

case $TEMPLATE in
    fastio)
        cp src/bin/template_fastio.rs "src/bin/${NAME}.rs"
        ;;
    simple)
        cp src/bin/template_simple.rs "src/bin/${NAME}.rs"
        ;;
    *)
        echo "Unknown template: $TEMPLATE (use fastio or simple)"
        exit 1
        ;;
esac

echo "Created src/bin/${NAME}.rs from template_${TEMPLATE}.rs"