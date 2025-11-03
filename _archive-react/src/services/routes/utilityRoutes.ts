/**
 * Utility Routes
 * Health check, dev endpoints, and root endpoint
 */

import { Router, Request, Response } from 'express';
import { ussdSessions } from '../ussd/index.js';

/**
 * Create utility routes
 */
export function createUtilityRoutes() {
  const router = Router();

  // Root endpoint
  router.get('/', (_req: Request, res: Response) => {
    res.json({
      message: 'AfriTokeni SMS & USSD Webhook Server',
      version: '3.0.0',
      description: 'Modular SMS & USSD service with 86% code reduction',
      endpoints: [
        'POST /api/send-sms',
        'POST /api/verify-code',
        'POST /api/webhook/sms',
        'POST /api/ussd',
        'POST /api/send-notification',
        'GET /health'
      ]
    });
  });

  // Health check endpoint
  router.get('/health', async (_req: Request, res: Response) => {
    try {
      const health = {
        status: 'healthy',
        timestamp: new Date().toISOString(),
        uptime: process.uptime(),
        services: {
          sms: 'operational',
          ussd: 'operational',
          datastore: 'operational'
        },
        sessions: {
          active: ussdSessions.size
        }
      };

      res.json(health);
    } catch (error) {
      res.status(500).json({
        status: 'unhealthy',
        error: error instanceof Error ? error.message : 'Unknown error'
      });
    }
  });

  // Development endpoint for PIN management
  router.post('/dev/pins/clear', async (_req: Request, res: Response) => {
    if (process.env.NODE_ENV !== 'production') {
      // Clear in-memory sessions
      ussdSessions.clear();
      
      res.json({
        success: true,
        message: 'Development data cleared',
        cleared: {
          sessions: 'all'
        }
      });
    } else {
      res.status(403).json({
        success: false,
        error: 'This endpoint is only available in development mode'
      });
    }
  });

  return router;
}
