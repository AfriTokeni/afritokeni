import { defineHook, type OnSetDoc } from "@junobuild/functions";
import { decodeDocData } from "@junobuild/functions/sdk";
import { Resend } from 'resend';

// Newsletter confirmation email function
export const onSetDoc = defineHook<OnSetDoc>({
  collections: ["email_subscriptions"],
  run: async (context) => {
    try {
      const data = decodeDocData(context.data.data.after.data) as any;
      const email = data.email as string;
      const subscribedAt = data.subscribedAt as string;
      
      console.log("New newsletter subscription detected:", { email, subscribedAt });
      
      // Send welcome email using Resend API
      await sendWelcomeEmail(email);
      
      console.log("Welcome email sent successfully to:", email);
    } catch (error) {
      console.error("Error sending welcome email:", error);
      throw error;
    }
  }
});

async function sendWelcomeEmail(email: string) {
  const RESEND_API_KEY = process.env.RESEND_API_KEY;
  
  if (!RESEND_API_KEY) {
    throw new Error("RESEND_API_KEY environment variable is not set");
  }

  const resend = new Resend(RESEND_API_KEY);

  const emailDomain = process.env.EMAIL_FROM_DOMAIN || "resend.dev";
  
  const result = await resend.emails.send({
    from: `noreply@${emailDomain}`,
    to: [email],
    subject: "Welcome to AfriTokeni Newsletter! 🚀",
    html: `
      <div style="font-family: Arial, sans-serif; max-width: 600px; margin: 0 auto; padding: 20px;">
        <div style="text-align: center; margin-bottom: 30px;">
          <h1 style="color: #1a1a1a; margin-bottom: 10px;">Welcome to AfriTokeni!</h1>
          <p style="color: #666; font-size: 16px;">Thank you for subscribing to our newsletter</p>
        </div>
        
        <div style="background: #f8f9fa; padding: 25px; border-radius: 8px; margin-bottom: 25px;">
          <h2 style="color: #1a1a1a; margin-bottom: 15px;">🎉 You're all set!</h2>
          <p style="color: #444; line-height: 1.6; margin-bottom: 15px;">
            Welcome to the AfriTokeni community! You'll now receive updates about:
          </p>
          <ul style="color: #444; line-height: 1.8; padding-left: 20px;">
            <li>Latest platform features and updates</li>
            <li>African fintech industry insights</li>
            <li>Exclusive community events and opportunities</li>
            <li>Tips for maximizing your AfriTokeni experience</li>
          </ul>
        </div>
        
        <div style="text-align: center; margin-bottom: 25px;">
          <a href="https://afritokeni.com" 
             style="display: inline-block; background: #1a1a1a; color: white; padding: 12px 25px; 
                    text-decoration: none; border-radius: 6px; font-weight: 500;">
            Visit AfriTokeni Platform
          </a>
        </div>
        
        <div style="border-top: 1px solid #eee; padding-top: 20px; text-align: center;">
          <p style="color: #888; font-size: 14px; margin-bottom: 10px;">
            Follow us for more updates:
          </p>
          <div style="margin-bottom: 15px;">
            <a href="#" style="color: #1a1a1a; text-decoration: none; margin: 0 10px;">Twitter</a>
            <a href="#" style="color: #1a1a1a; text-decoration: none; margin: 0 10px;">LinkedIn</a>
            <a href="#" style="color: #1a1a1a; text-decoration: none; margin: 0 10px;">GitHub</a>
          </div>
          <p style="color: #999; font-size: 12px;">
            If you didn't subscribe to this newsletter, you can safely ignore this email.
          </p>
        </div>
      </div>
    `
  });

  return result;
}
