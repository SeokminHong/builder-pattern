cd "$(dirname "$0")/../test-no-future"
for example in examples/*.rs
do
    e=${example##*/}
    cargo run --example ${e%.*}
    result=$?
    if [[ $e == fail-* ]]; then
        if [[ $result -eq 0 ]]; then
            echo "Error: example '$e' should fail!"
            exit 1
        fi
    elif [[ $result -ne 0 ]]; then
        exit 1
    fi
done;
