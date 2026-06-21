## Some website
---
- Frontend: vue
- Backend : flask

## Developer Notes
---
> bored on a weekend

### Initial Setup
---
Start with a template vue project,
```bash
npm create vue@latest
```
fix the asset paths in `vite.config.js` by adding `base: '/static/'` to the config:
```
import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'

// https://vite.dev/config/
export default defineConfig({
  plugins: [vue()],
  base: '/static/'
})
```

and remove images as well default favicon and icons from template project.

### Build
---
Building the frontend
```bash
cd frontend
npm run build
```
you'll get `frontend/dist`

Copy build to flask 


`frontend/dist/index.html` -> `backend/website/templates/index.html` 


and `frontend/dist/assets/*` -> `backend/website/static/assets/*`.
