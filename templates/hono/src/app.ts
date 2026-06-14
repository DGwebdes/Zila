import { Hono } from 'hono'
import { cors } from 'hono/cors'
import { secureHeaders } from 'hono/secure-headers'
import { router } from './routes/index.js'

const app = new Hono()

// Middleware
app.use('*', cors())
app.use('*', secureHeaders())

// Routes
app.route('/api', router)

export default app
