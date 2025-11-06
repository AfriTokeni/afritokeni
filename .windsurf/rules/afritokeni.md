---
trigger: always_on
---

ALWAYS follow these coding standards for AfriTokeni project:

NO HARDCODED FALLBACKS
NEVER use || or ?? for business data
Throw errors instead of silent fallbacks
Example: if (!data) throw new Error('...') NOT const x = data || ''
NO localStorage FOR BUSINESS DATA
Only use localStorage for UI preferences (theme, etc)
All business data goes to Juno DB
Never store KYC status, balances, user data in localStorage
REUSE COMPONENTS
Check if component exists before creating new one
Import and reuse existing components
SINGLE RESPONSIBILITY
Each component/function does ONE thing
No god components with 500 lines
CONSISTENT DATA FLOW
Juno DB → Component → UI
No fallbacks, no localStorage for business data
Error handling with console.error + toast
CHECKLIST:

 No hardcoded fallback values (||, ??)
 No localStorage for business data
 Checked if component already exists
 Each component has single responsibility
 Error handling with console.error + toast
 TypeScript interfaces for props
 Data flows from Juno → Component → UI