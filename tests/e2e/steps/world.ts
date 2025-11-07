import { World, setWorldConstructor } from '@cucumber/cucumber';

export interface UssdRequest {
  sessionId: string;
  serviceCode: string;
  phoneNumber: string;
  text: string;
}

export class UssdWorld extends World {
  public phoneNumber: string = '';
  public pin: string = '';
  public sessionId: string = '';
  public lastResponse: string = '';
  public inputHistory: string[] = [];
  
  // Satellite endpoint - will be configurable
  public satelliteUrl: string = process.env.SATELLITE_URL || 'http://localhost:5987';
  
  async callUssdEndpoint(text: string): Promise<string> {
    const request: UssdRequest = {
      sessionId: this.sessionId || `test-${Date.now()}`,
      serviceCode: '*229#',
      phoneNumber: this.phoneNumber,
      text: text
    };
    
    // Store session ID for subsequent calls
    if (!this.sessionId) {
      this.sessionId = request.sessionId;
    }
    
    // Make HTTP POST to satellite's USSD endpoint
    const response = await fetch(`${this.satelliteUrl}/api/ussd`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify(request)
    });
    
    if (!response.ok) {
      throw new Error(`USSD endpoint returned ${response.status}: ${await response.text()}`);
    }
    
    const responseText = await response.text();
    this.lastResponse = responseText;
    this.inputHistory.push(text);
    
    return responseText;
  }
  
  // Build cumulative input (AfricasTalking sends full history)
  getCumulativeInput(): string {
    return this.inputHistory.join('*');
  }
}

setWorldConstructor(UssdWorld);
