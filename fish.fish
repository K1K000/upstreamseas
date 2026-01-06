#!/usr/bin/env sh 

curl http://127.0.0.1:8000/book
echo ""
curl http://127.0.0.1:8000/borrow
echo ""

curl -X POST http://127.0.0.1:8000/student -H "Content-Type: application/json" -d '{"name": "peter", "email": "emal", "has_card": true}'                             
echo ""
curl -X POST http://127.0.0.1:8000/book -H "Content-Type: application/json" -d '{"name": "peters book", "available": 1000 }'
echo ""
curl -X POST http://127.0.0.1:8000/borrow -H "Content-Type: application/json" -d '{"student_id": 1, "book_id": 1, "borrow_lenght": 10}'                             
echo ""
curl -X POST http://127.0.0.1:8000/student -H "Content-Type: application/json" -d '{"name": "john", "email": "emal", "has_card": true}'                             
echo ""
curl -X POST http://127.0.0.1:8000/book -H "Content-Type: application/json" -d '{"name": "john book", "available": 1000 }'
echo ""
curl -X POST http://127.0.0.1:8000/borrow -H "Content-Type: application/json" -d '{"student_id": 1, "book_id": 2, "borrow_lenght": 10}'                             
echo ""
curl -X POST http://127.0.0.1:8000/student -H "Content-Type: application/json" -d '{"name": "peter", "email": "emal", "has_card": true}'                             
echo ""
curl -X POST http://127.0.0.1:8000/book -H "Content-Type: application/json" -d '{"name": "peters book", "available": 1000 }'
echo ""
curl -X POST http://127.0.0.1:8000/borrow -H "Content-Type: application/json" -d '{"student_id": 1, "book_id": 2, "borrow_lenght": 10}'                             
echo ""
curl -X POST http://127.0.0.1:8000/student -H "Content-Type: application/json" -d '{"name": "peter", "email": "emal", "has_card": true}'                             
echo ""
curl -X POST http://127.0.0.1:8000/book -H "Content-Type: application/json" -d '{"name": "peters book", "available": 1000 }'
echo ""
curl -X POST http://127.0.0.1:8000/borrow -H "Content-Type: application/json" -d '{"student_id": 1, "book_id": 2, "borrow_lenght": 10}'                             
echo ""
# curl -X PUT http://127.0.0.1:8000/book/1 -H "Content-Type: application/json" -d '{"name": "peters book", "available": 18 }'
# echo ""
# curl http://127.0.0.1:8000/borrow
# echo ""
# curl -X POST http://127.0.0.1:8000/author -H "Content-Type: application/json" -d '{"name": "also peter", "birthplace": "paks"}'                             
# echo ""
# curl -X POST http://127.0.0.1:8000/book_author -H "Content-Type: application/json" -d '{"author_id": 1, "book_id": 1}'                             





