rm backend/src/static/assets/*
cd frontend
npm run build 
cd ..
cp frontend/dist/index.html backend/src/templates/
cp frontend/dist/assets/* backend/src/static/assets/

