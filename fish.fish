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

nl curl -X POST http://127.0.0.1:8000/ticket -H "Content-Type: application/json" -d '{"student_id": 1,  "end_date": "2026-08-18"}'                             
nl curl -X POST http://127.0.0.1:8000/ticket -H "Content-Type: application/json" -d '{"student_id": 2,  "end_date": "2026-08-18"}'                             
nl curl -X POST http://127.0.0.1:8000/ticket -H "Content-Type: application/json" -d '{"student_id": 3,  "end_date": "2026-08-18"}'                             
nl curl -X POST http://127.0.0.1:8000/ticket -H "Content-Type: application/json" -d '{"student_id": 4,  "end_date": "2026-08-18"}'                             

nl curl -X POST http://127.0.0.1:8000/book -H "Content-Type: application/json" -d '{"name": "the king in yellow","max_borrow": 100, "all_available": 1000, "description":"good", "release": "2026-01-15"}'
nl curl -X POST http://127.0.0.1:8000/book -H "Content-Type: application/json" -d '{"name": "i married a woman", "max_borrow": 100,"all_available": 1000, "description":"good", "release": "2026-01-15"}'
nl curl -X POST http://127.0.0.1:8000/book -H "Content-Type: application/json" -d '{"name": "eternal love","max_borrow": 100, "all_available": 1000, "description":"good", "release": "2026-01-15"}'

nl curl -X POST http://127.0.0.1:8000/borrow -H "Content-Type: application/json" -d '{"student_id": 1, "book_id": 2 }'
nl curl -X POST http://127.0.0.1:8000/borrow -H "Content-Type: application/json" -d '{"student_id": 4, "book_id": 1 }'
nl curl -X POST http://127.0.0.1:8000/borrow -H "Content-Type: application/json" -d '{"student_id": 2, "book_id": 1 }'
nl curl -X POST http://127.0.0.1:8000/borrow -H "Content-Type: application/json" -d '{"student_id": 2, "book_id": 3 }'
nl curl -X POST http://127.0.0.1:8000/borrow -H "Content-Type: application/json" -d '{"student_id": 3, "book_id": 2 }'
nl curl -X POST http://127.0.0.1:8000/borrow -H "Content-Type: application/json" -d '{"student_id": 4, "book_id": 2 }'

# nl curl -X PUT http://127.0.0.1:8000/borrow/1 -H "Content-Type: application/json" -d '{"student_id": 1, "book_id": 2,  "end": "2006-01-01" }'


nl curl http://127.0.0.1:8000/student/1/active_books

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





