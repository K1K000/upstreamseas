#!/usr/bin/env fish

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

nl curl -X POST http://127.0.0.1:8000/book_category -H "Content-Type: application/json" -d '{"book_id": 1, "category_id": 1}'
nl curl -X POST http://127.0.0.1:8000/book_category -H "Content-Type: application/json" -d '{"book_id": 1, "category_id": 3}'
nl curl -X POST http://127.0.0.1:8000/book_category -H "Content-Type: application/json" -d '{"book_id": 2, "category_id": 3}'
nl curl -X POST http://127.0.0.1:8000/book_category -H "Content-Type: application/json" -d '{"book_id": 2, "category_id": 4}'
nl curl -X POST http://127.0.0.1:8000/book_category -H "Content-Type: application/json" -d '{"book_id": 3, "category_id": 2}'
nl curl -X POST http://127.0.0.1:8000/book_category -H "Content-Type: application/json" -d '{"book_id": 3, "category_id": 3}'



echo "book 1 categories"
curl http://127.0.0.1:8000/book/categories/1
