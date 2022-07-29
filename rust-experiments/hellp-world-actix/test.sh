clear

curl -s \
    -w '\n' \
    -H 'Content-Type: application/json' \
    http://localhost:8080

curl -s \
    -w '\n' \
    -H 'Content-Type: application/json' \
    http://localhost:8080/hey

curl -s \
    -w '\n' \
    -H 'Content-Type: application/json' \
    -d '{"say":"hi"}' \
    http://localhost:8080/echo

curl -s \
    -w '\n' \
    -H 'Content-Type: application/json' \
    http://localhost:8080/app/index.html

curl -s \
    -w '\n' \
    -H 'Content-Type: application/json' \
    http://localhost:8080/print-count

curl -s \
    -w '\n' \
    -H 'Content-Type: application/json' \
    http://localhost:8080/counter

curl -s \
    -w '\n' \
    http://localhost:8080/app/guard

curl -s \
    -w '\n' \
    -H 'x-guarded: secret' \
    http://localhost:8080/app/guard

curl -s \
    -w '\n' \
    http://localhost:8080/responder




