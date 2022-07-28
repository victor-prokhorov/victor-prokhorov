clear
curl --head http://localhost:3000
echo '\n'
# curl -X POST -H 'Content-Type: application/json' -d '{"username": "test_username", "random_field": "test_random_field","number":0,"optional":"test_optional"}' http://localhost:3000/users
# curl -X POST -H 'Content-Type: application/json' -d @'./user.json' http://localhost:3000/users
# curl -X GET -H 'Content-Type: application/json' http://localhost:3000/protected

curl -s \
    -w '\n' \
    -H 'Content-Type: application/json' \
    -d '{"client_id":"foo","client_secret":"bar"}' \
    http://localhost:3000/authorize

curl -s \
    -w '\n' \
    -H 'Content-Type: application/json' \
    -H 'Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJiQGIuY29tIiwiY29tcGFueSI6IkFDTUUiLCJleHAiOjEwMDAwMDAwMDAwfQ.M3LAZmrzUkXDC1q5mSzFAs_kJrwuKz3jOoDmjJ0G4gM' \
    http://localhost:3000/protected

curl -s \
    -w '\n' \
    -H 'Content-Type: application/json' \
    -H 'Authorization: Bearer blahblahblah' \
    http://localhost:3000/protected
