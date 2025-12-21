#!/usr/bin/env sh 

curl -X POST http://127.0.0.1:8000/student -H "Content-Type: application/json" -d '{"name": "peter", "email": "emal"}'                             
echo ""
curl -X POST http://127.0.0.1:8000/book -H "Content-Type: application/json" -d '{"name": "peters book", "available": 1000 }'                             
echo ""
curl -X POST http://127.0.0.1:8000/borrow -H "Content-Type: application/json" -d '{"student_id": 1, "book_id": 1}'                             
echo ""
curl http://127.0.0.1:8000/borrow
echo ""
curl -X POST http://127.0.0.1:8000/author -H "Content-Type: application/json" -d '{"name": "also peter", "birthplace": "paks"}'                             
echo ""
curl -X POST http://127.0.0.1:8000/book_author -H "Content-Type: application/json" -d '{"author_id": 1, "book_id": 1}'                             

echo "hello fish"
