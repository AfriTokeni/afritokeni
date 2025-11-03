/**
 * Routes Index
 * Central export for all route modules
 */

export { createSMSRoutes, verificationCodes } from './smsRoutes.js';
export { createUSSDRoutes, setSMSNotificationFunction } from './ussdRoutes.js';
export { createNotificationRoutes } from './notificationRoutes.js';
export { createUtilityRoutes } from './utilityRoutes.js';
