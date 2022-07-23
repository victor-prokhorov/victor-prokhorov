clear
curl --head http://localhost:3000
# curl -X POST -H 'Content-Type: application/json' -d '{"username": "test_username", "random_field": "test_random_field","number":0,"optional":"test_optional"}' http://localhost:3000/users
curl -X POST -H 'Content-Type: application/json' -d @'./user.json' http://localhost:3000/users
echo ''
