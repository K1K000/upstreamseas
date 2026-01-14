#!/usr/bin/env fish

curl http://127.0.0.1:8000/book
echo ""
curl http://127.0.0.1:8000/borrow
echo ""

function nl 
  $argv && echo ""
end

nl curl -X POST http://127.0.0.1:8000/student -H "Content-Type: application/json" -d '{"name": "peter2", "email": "emal", "has_card": true}'                             
nl curl -X POST http://127.0.0.1:8000/student -H "Content-Type: application/json" -d '{"name": "peter3", "email": "emal", "has_card": true}'                             
nl curl -X POST http://127.0.0.1:8000/student -H "Content-Type: application/json" -d '{"name": "peter1", "email": "emal", "has_card": true}'                             
nl curl -X POST http://127.0.0.1:8000/student -H "Content-Type: application/json" -d '{"name": "pedobot", "email": "emal", "has_card": true}'                             

nl curl -X POST http://127.0.0.1:8000/ticket -H "Content-Type: application/json" -d '{"student_id": 1,  "end_date": "2026-01-18"}'                             
nl curl -X POST http://127.0.0.1:8000/ticket -H "Content-Type: application/json" -d '{"student_id": 2,  "end_date": "2026-01-18"}'                             
nl curl -X POST http://127.0.0.1:8000/ticket -H "Content-Type: application/json" -d '{"student_id": 3,  "end_date": "2026-01-18"}'                             
nl curl -X POST http://127.0.0.1:8000/ticket -H "Content-Type: application/json" -d '{"student_id": 4,  "end_date": "2026-01-18"}'                             

nl curl -X POST http://127.0.0.1:8000/book -H "Content-Type: application/json" -d '{"name": "peters book", "available": 1000 }'
nl curl -X POST http://127.0.0.1:8000/book -H "Content-Type: application/json" -d '{"name": "peters book", "available": 1000 }'
nl curl -X POST http://127.0.0.1:8000/book -H "Content-Type: application/json" -d '{"name": "peters book", "available": 1000 }'

nl curl -X POST http://127.0.0.1:8000/borrow -H "Content-Type: application/json" -d '{"student_id": 4, "book_id": 1, "borrow_lenght": 10}'
nl curl -X POST http://127.0.0.1:8000/borrow -H "Content-Type: application/json" -d '{"student_id": 2, "book_id": 1, "borrow_lenght": 10}'
nl curl -X POST http://127.0.0.1:8000/borrow -H "Content-Type: application/json" -d '{"student_id": 2, "book_id": 3, "borrow_lenght": 10}'
nl curl -X POST http://127.0.0.1:8000/borrow -H "Content-Type: application/json" -d '{"student_id": 3, "book_id": 2, "borrow_lenght": 10}'
nl curl -X POST http://127.0.0.1:8000/borrow -H "Content-Type: application/json" -d '{"student_id": 4, "book_id": 2, "borrow_lenght": 10}'
nl curl -X POST http://127.0.0.1:8000/borrow -H "Content-Type: application/json" -d '{"student_id": 1, "book_id": 2, "borrow_lenght": 10}'


#
#
#
#
#
# curl -X PUT http://127.0.0.1:8000/book/1 -H "Content-Type: application/json" -d '{"name": "peters book", "available": 18 }'
# echo ""
# curl http://127.0.0.1:8000/borrow
# echo ""
# curl -X POST http://127.0.0.1:8000/author -H "Content-Type: application/json" -d '{"name": "also peter", "birthplace": "paks"}'                             
# echo ""
# curl -X POST http://127.0.0.1:8000/book_author -H "Content-Type: application/json" -d '{"author_id": 1, "book_id": 1}'                             





