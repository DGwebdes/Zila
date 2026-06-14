import { Hono } from 'hono'
import { cors } from 'hono/cors'
import { secureHeaders } from 'hono/secure-headers'
import { router } from './routes/index.js'
import { errorHandler } from './middleware/errorHandler.js'
import { notFound } from './middleware/notFound.js'
import { logger } from './middleware/logger.js'

const app = new Hono()

app.use('*', cors())
app.use('*', secureHeaders())
app.use('*', logger)
app.use('*', errorHandler)

app.route('/api', router)

app.notFound(notFound)

export default app
