#!/usr/bin/env fish

function nl 
  $argv && echo ""
end

nl curl -X POST http://127.0.0.1:8000/category -H "Content-Type: application/json" -d '{"name": "horror", "description": "very scary book" }'                             
nl curl -X POST http://127.0.0.1:8000/category -H "Content-Type: application/json" -d '{"name": "action", "description": "super fast" }'                             
nl curl -X POST http://127.0.0.1:8000/category -H "Content-Type: application/json" -d '{"name": "romance", "description": "smooch" }'                             
nl curl -X POST http://127.0.0.1:8000/category -H "Content-Type: application/json" -d '{"name": "yuri", "description": "peak" }'                             

nl curl -X POST http://127.0.0.1:8000/author -H "Content-Type: application/json" -d '{"name": "yuri", "birthplace": "paks", "birthdate": "2026-01-15",  "description": "peak" }'                             

nl curl -X POST http://127.0.0.1:8000/book -H "Content-Type: application/json" -d '{"name": "the king in yellow","max_borrow": 1000, "all_available": 1000, "description":"good", "release": "2026-01-15"}'
nl curl -X POST http://127.0.0.1:8000/book -H "Content-Type: application/json" -d '{"name": "i married a woman","max_borrow": 1000, "all_available": 1000, "description":"good", "release": "2026-01-15"}'
nl curl -X POST http://127.0.0.1:8000/book -H "Content-Type: application/json" -d '{"name": "eternal love", "all_available": 1000,"max_borrow": 1000, "description":"good", "release": "2026-01-15"}'
nl curl -X PUT http://127.0.0.1:8000/book/3 -H "Content-Type: application/json" -d '{"name": "Janee you are early","max_borrow": 1000, "deleted": false, "available": 1000,  "all_available": 1000, "description":"good", "release": "2026-01-15"}'

nl curl -X POST http://127.0.0.1:8000/book_category -H "Content-Type: application/json" -d '{"book_id": 1, "category_id": 1}'
nl curl -X POST http://127.0.0.1:8000/book_category -H "Content-Type: application/json" -d '{"book_id": 1, "category_id": 3}'
nl curl -X POST http://127.0.0.1:8000/book_category -H "Content-Type: application/json" -d '{"book_id": 2, "category_id": 3}'
nl curl -X POST http://127.0.0.1:8000/book_category -H "Content-Type: application/json" -d '{"book_id": 2, "category_id": 4}'
nl curl -X POST http://127.0.0.1:8000/book_category -H "Content-Type: application/json" -d '{"book_id": 3, "category_id": 2}'
nl curl -X POST http://127.0.0.1:8000/book_category -H "Content-Type: application/json" -d '{"book_id": 3, "category_id": 3}'



echo "book 1 categories"
curl http://127.0.0.1:8000/book/id/1/categories/
