import { AfricanCurrency } from '../types/currency';

export interface LocationData {
  latitude: number;
  longitude: number;
  address?: string;
  city?: string;
  region?: string;
  country?: string;
  accessibility: 'urban' | 'suburban' | 'rural' | 'remote';
  population?: number;
}

export interface AgentLocation extends LocationData {
  agentId: string;
  operatingRadius: number; // in kilometers
  lastActiveAt: Date;
  serviceTypes: ('cash_in' | 'cash_out' | 'bitcoin_exchange')[];
}

export interface TransactionRequest {
  amount: number;
  currency: AfricanCurrency;
  type: 'cash_in' | 'cash_out' | 'bitcoin_buy' | 'bitcoin_sell';
  customerLocation: LocationData;
  urgency: 'standard' | 'express' | 'emergency';
  timeOfDay: 'morning' | 'afternoon' | 'evening' | 'night';
  dayOfWeek: 'weekday' | 'weekend';
}

export interface FeeCalculation {
  baseFee: number; // Platform base fee (1-2%)
  distanceFee: number; // Distance-based fee
  accessibilityFee: number; // Location accessibility fee
  demandFee: number; // Supply/demand adjustment
  urgencyFee: number; // Time-based urgency fee
  totalFeePercentage: number;
  totalFeeAmount: number;
  agentCommission: number;
  platformRevenue: number;
  breakdown: {
    description: string;
    percentage: number;
    amount: number;
  }[];
}

export class DynamicFeeService {
  // Base fee structure
  private static readonly BASE_FEE = 0.015; // 1.5% base platform fee
  private static readonly MIN_TOTAL_FEE = 0.02; // 2% minimum total fee
  private static readonly MAX_TOTAL_FEE = 0.12; // 12% maximum total fee

  // Distance-based fee calculation
  private static readonly DISTANCE_TIERS = {
    local: { maxKm: 5, feePercentage: 0.005 }, // 0.5% for local (within 5km)
    nearby: { maxKm: 20, feePercentage: 0.01 }, // 1% for nearby (5-20km)
    regional: { maxKm: 50, feePercentage: 0.02 }, // 2% for regional (20-50km)
    distant: { maxKm: 100, feePercentage: 0.035 }, // 3.5% for distant (50-100km)
    remote: { maxKm: Infinity, feePercentage: 0.05 } // 5% for remote (100km+)
  };

  // Accessibility-based fee multipliers
  private static readonly ACCESSIBILITY_MULTIPLIERS = {
    urban: 1.0, // No additional fee for urban areas
    suburban: 1.2, // 20% increase for suburban
    rural: 1.5, // 50% increase for rural areas
    remote: 2.0 // 100% increase for remote villages
  };

  // Time-based urgency multipliers
  private static readonly URGENCY_MULTIPLIERS = {
    standard: 1.0, // No additional fee
    express: 1.3, // 30% increase for express (within 2 hours)
    emergency: 1.8 // 80% increase for emergency (within 30 minutes)
  };

  // Time of day multipliers
  private static readonly TIME_MULTIPLIERS = {
    morning: 1.0, // 6 AM - 12 PM
    afternoon: 1.0, // 12 PM - 6 PM
    evening: 1.1, // 6 PM - 10 PM (10% increase)
    night: 1.4 // 10 PM - 6 AM (40% increase for night service)
  };

  // Weekend multiplier
  private static readonly WEEKEND_MULTIPLIER = 1.15; // 15% increase on weekends

  /**
   * Calculate the distance between two geographic points using Haversine formula
   */
  static calculateDistance(
    lat1: number, 
    lon1: number, 
    lat2: number, 
    lon2: number
  ): number {
    const R = 6371; // Earth's radius in kilometers
    const dLat = this.toRadians(lat2 - lat1);
    const dLon = this.toRadians(lon2 - lon1);
    
    const a = 
      Math.sin(dLat / 2) * Math.sin(dLat / 2) +
      Math.cos(this.toRadians(lat1)) * Math.cos(this.toRadians(lat2)) *
      Math.sin(dLon / 2) * Math.sin(dLon / 2);
    
    const c = 2 * Math.atan2(Math.sqrt(a), Math.sqrt(1 - a));
    return R * c;
  }

  private static toRadians(degrees: number): number {
    return degrees * (Math.PI / 180);
  }

  /**
   * Find the best agent for a transaction based on distance and availability
   */
  static findBestAgent(
    customerLocation: LocationData,
    availableAgents: AgentLocation[],
    serviceType: 'cash_in' | 'cash_out' | 'bitcoin_exchange'
  ): { agent: AgentLocation; distance: number } | null {
    const suitableAgents = availableAgents.filter(agent => 
      agent.serviceTypes.includes(serviceType)
    );

    if (suitableAgents.length === 0) return null;

    let bestAgent: AgentLocation | null = null;
    let shortestDistance = Infinity;

    for (const agent of suitableAgents) {
      const distance = this.calculateDistance(
        customerLocation.latitude,
        customerLocation.longitude,
        agent.latitude,
        agent.longitude
      );

      // Check if customer is within agent's operating radius
      if (distance <= agent.operatingRadius && distance < shortestDistance) {
        bestAgent = agent;
        shortestDistance = distance;
      }
    }

    return bestAgent ? { agent: bestAgent, distance: shortestDistance } : null;
  }

  /**
   * Calculate dynamic fee based on all factors
   */
  static calculateDynamicFee(
    request: TransactionRequest,
    agentDistance: number,
    _agentLocation: LocationData
  ): FeeCalculation {
    const breakdown: FeeCalculation['breakdown'] = [];

    // 1. Base platform fee
    const baseFee = this.BASE_FEE;
    breakdown.push({
      description: 'Platform base fee',
      percentage: baseFee,
      amount: request.amount * baseFee
    });

    // 2. Distance-based fee
    let distanceFee = 0;
    for (const [tier, config] of Object.entries(this.DISTANCE_TIERS)) {
      if (agentDistance <= config.maxKm) {
        distanceFee = config.feePercentage;
        breakdown.push({
          description: `Distance fee (${tier}: ${agentDistance.toFixed(1)}km)`,
          percentage: distanceFee,
          amount: request.amount * distanceFee
        });
        break;
      }
    }

    // 3. Accessibility fee (based on customer location)
    const accessibilityMultiplier = this.ACCESSIBILITY_MULTIPLIERS[request.customerLocation.accessibility];
    const accessibilityFee = distanceFee * (accessibilityMultiplier - 1);
    if (accessibilityFee > 0) {
      breakdown.push({
        description: `${request.customerLocation.accessibility} area fee`,
        percentage: accessibilityFee,
        amount: request.amount * accessibilityFee
      });
    }

    // 4. Urgency fee
    const urgencyMultiplier = this.URGENCY_MULTIPLIERS[request.urgency];
    const urgencyFee = (baseFee + distanceFee) * (urgencyMultiplier - 1);
    if (urgencyFee > 0) {
      breakdown.push({
        description: `${request.urgency} service fee`,
        percentage: urgencyFee,
        amount: request.amount * urgencyFee
      });
    }

    // 5. Time-based fee
    const timeMultiplier = this.TIME_MULTIPLIERS[request.timeOfDay];
    const weekendMultiplier = request.dayOfWeek === 'weekend' ? this.WEEKEND_MULTIPLIER : 1.0;
    const combinedTimeMultiplier = timeMultiplier * weekendMultiplier;
    const timeFee = (baseFee + distanceFee) * (combinedTimeMultiplier - 1);
    if (timeFee > 0) {
      const timeDescription = request.dayOfWeek === 'weekend' 
        ? `${request.timeOfDay} weekend service`
        : `${request.timeOfDay} service`;
      breakdown.push({
        description: timeDescription,
        percentage: timeFee,
        amount: request.amount * timeFee
      });
    }

    // 6. Market demand adjustment (simplified for POC)
    const demandFee = this.calculateDemandAdjustment(request.customerLocation, request.type);
    if (demandFee !== 0) {
      breakdown.push({
        description: demandFee > 0 ? 'High demand adjustment' : 'Low demand discount',
        percentage: demandFee,
        amount: request.amount * demandFee
      });
    }

    // Calculate totals
    let totalFeePercentage = baseFee + distanceFee + accessibilityFee + urgencyFee + timeFee + demandFee;
    
    // Apply min/max limits
    totalFeePercentage = Math.max(this.MIN_TOTAL_FEE, Math.min(this.MAX_TOTAL_FEE, totalFeePercentage));
    
    const totalFeeAmount = request.amount * totalFeePercentage;
    
    // Agent gets 70% of the total fee, platform keeps 30%
    const agentCommission = totalFeeAmount * 0.7;
    const platformRevenue = totalFeeAmount * 0.3;

    return {
      baseFee,
      distanceFee: distanceFee + accessibilityFee,
      accessibilityFee,
      demandFee,
      urgencyFee: urgencyFee + timeFee,
      totalFeePercentage,
      totalFeeAmount,
      agentCommission,
      platformRevenue,
      breakdown
    };
  }

  /**
   * Calculate demand-based fee adjustment (simplified for POC)
   */
  private static calculateDemandAdjustment(
    location: LocationData,
    _transactionType: string
  ): number {
    // Simplified demand calculation based on location accessibility
    // In production, this would use real-time data on agent availability and transaction volume
    
    if (location.accessibility === 'remote') {
      // High demand adjustment for remote areas (fewer agents)
      return 0.01; // +1%
    } else if (location.accessibility === 'urban') {
      // Small discount for urban areas (more agents, more competition)
      return -0.005; // -0.5%
    }
    
    return 0; // No adjustment for suburban/rural
  }

  /**
   * Get fee estimate for display to customer before transaction
   */
  static getFeeEstimate(
    amount: number,
    currency: AfricanCurrency,
    customerLocation: LocationData,
    transactionType: 'cash_in' | 'cash_out' | 'bitcoin_buy' | 'bitcoin_sell',
    urgency: 'standard' | 'express' | 'emergency' = 'standard'
  ): {
    estimatedFeeRange: { min: number; max: number };
    factors: string[];
  } {
    const now = new Date();
    const hour = now.getHours();
    const timeOfDay = hour < 6 ? 'night' : hour < 12 ? 'morning' : hour < 18 ? 'afternoon' : hour < 22 ? 'evening' : 'night';
    const dayOfWeek = now.getDay() === 0 || now.getDay() === 6 ? 'weekend' : 'weekday';

    // Calculate fee for different distance scenarios
    const nearbyRequest: TransactionRequest = {
      amount,
      currency,
      type: transactionType,
      customerLocation,
      urgency,
      timeOfDay: timeOfDay as any,
      dayOfWeek: dayOfWeek as any
    };

    // Mock agent location for estimation
    const nearbyAgent: LocationData = {
      ...customerLocation,
      accessibility: 'urban'
    };
    const remoteAgent: LocationData = {
      ...customerLocation,
      accessibility: 'remote'
    };

    const nearbyFee = this.calculateDynamicFee(nearbyRequest, 5, nearbyAgent); // 5km away
    const remoteFee = this.calculateDynamicFee(nearbyRequest, 50, remoteAgent); // 50km away

    const factors = [
      `Base fee: ${(this.BASE_FEE * 100).toFixed(1)}%`,
      `Distance: 0.5-5% depending on agent location`,
      `Location: ${customerLocation.accessibility} area`,
      `Service: ${urgency}`,
      `Time: ${timeOfDay}${dayOfWeek === 'weekend' ? ' weekend' : ''}`
    ];

    return {
      estimatedFeeRange: {
        min: nearbyFee.totalFeeAmount,
        max: remoteFee.totalFeeAmount
      },
      factors
    };
  }

  /**
   * Format fee breakdown for display
   */
  static formatFeeBreakdown(calculation: FeeCalculation): string {
    let breakdown = `Fee Breakdown:\n`;
    calculation.breakdown.forEach(item => {
      breakdown += `• ${item.description}: ${(item.percentage * 100).toFixed(2)}% (${item.amount.toFixed(0)})\n`;
    });
    breakdown += `\nTotal Fee: ${(calculation.totalFeePercentage * 100).toFixed(2)}% (${calculation.totalFeeAmount.toFixed(0)})\n`;
    breakdown += `Agent Commission: ${calculation.agentCommission.toFixed(0)}\n`;
    breakdown += `Platform Fee: ${calculation.platformRevenue.toFixed(0)}`;
    return breakdown;
  }
}

export default DynamicFeeService;
