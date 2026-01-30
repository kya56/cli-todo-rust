#!/usr/bin/env bash

set -e

BASE_URL="http://127.0.0.1:8080/todos"

request() {
    response=$(curl -s -w "\n%{http_code}" "$@")
    body=$(echo "$response" | sed '$d')
    status=$(echo "$response" | tail -n1)

    if echo "$body" | jq -e . >/dev/null 2>&1; then
        echo "$body" | jq .
    else
        echo "$body"
    fi

    echo "HTTP_STATUS: $status"
    echo
}

echo "Resetting todo state"
rm -f resource/todo.json
request -X DELETE "$BASE_URL/reset"

echo "Creating todos"
request -X POST "$BASE_URL" \
    -H "Content-Type: application/json" \
    -d '{"title":"First task"}'

echo "Create second todo"
request -X POST "$BASE_URL" \
    -H "Content-Type: application/json" \
    -d '{"title":"Second task"}'

echo "Create third todo"
request -X POST "$BASE_URL" \
    -H "Content-Type: application/json" \
    -d '{"title":"Third task"}'

echo "Listing todos"
request "$BASE_URL"

echo "Mark First task done"
request -X POST "$BASE_URL/1/mark-done"

echo "Mark Second task done"
request -X POST "$BASE_URL/2/mark-done"

echo "Listing completed todos"
request "$BASE_URL?mode=done"

echo "Undo Second task done"
request -X POST "$BASE_URL/2/undo-done"

echo "Listing completed todos"
request "$BASE_URL?mode=done"

echo "Listing open todos"
request "$BASE_URL?mode=todo"

echo "Update third todo"
request -X PUT "$BASE_URL/3" \
    -H "Content-Type: application/json" \
    -d '{"title":"Updated third task"}'

echo "Listing todos"
request "$BASE_URL"

echo "Deleting first todo"
request -X DELETE "$BASE_URL/1"

echo "Listing todos"
request "$BASE_URL"
