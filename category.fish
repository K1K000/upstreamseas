#!/usr/bin/env fish

# curl http://127.0.0.1:8000/book
# echo ""
# curl http://127.0.0.1:8000/borrow
# echo ""

function nl 
  $argv && echo ""
end

nl curl -X POST http://127.0.0.1:8000/category -H "Content-Type: application/json" -d '{"name": "horror", "description": "very scary book" }'                             
nl curl -X POST http://127.0.0.1:8000/category -H "Content-Type: application/json" -d '{"name": "action", "description": "super fast" }'                             
nl curl -X POST http://127.0.0.1:8000/category -H "Content-Type: application/json" -d '{"name": "romance", "description": "smooch" }'                             
nl curl -X POST http://127.0.0.1:8000/category -H "Content-Type: application/json" -d '{"name": "yuri", "description": "peak" }'                             

nl curl -X POST http://127.0.0.1:8000/book -H "Content-Type: application/json" -d '{"name": "the king in yellow", "available": 1000 }'
nl curl -X POST http://127.0.0.1:8000/book -H "Content-Type: application/json" -d '{"name": "i married a woman", "available": 1000 }'
nl curl -X POST http://127.0.0.1:8000/book -H "Content-Type: application/json" -d '{"name": "eternal love", "available": 1000 }'

nl curl -X POST http://127.0.0.1:8000/ -H "Content-Type: application/json" -d '{"student_id": 4, "book_id": 1, "borrow_lenght": 10}'
nl curl -X POST http://127.0.0.1:8000/borrow -H "Content-Type: application/json" -d '{"student_id": 4, "book_id": 1, "borrow_lenght": 10}'
nl curl -X POST http://127.0.0.1:8000/borrow -H "Content-Type: application/json" -d '{"student_id": 4, "book_id": 1, "borrow_lenght": 10}'
nl curl -X POST http://127.0.0.1:8000/borrow -H "Content-Type: application/json" -d '{"student_id": 4, "book_id": 1, "borrow_lenght": 10}'

nl curl http://127.0.0.1:8000/book/top/3
