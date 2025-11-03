# AfriTokeni Coding Standards

## 🔥 CRITICAL RULES - NEVER VIOLATE THESE

### 1. **NO HARDCODED DATA - EVER**
❌ **FORBIDDEN:**
```typescript
// WRONG - Hardcoded fallbacks
const userData = {
  firstName: data?.firstName || '',
  kycStatus: data?.kycStatus || 'pending',
  balance: data?.balance || 0
};
```

✅ **CORRECT:**
```typescript
// RIGHT - Use exact data from backend or show error
if (!doc) {
  console.error('❌ DATA ERROR:', error);
  toast.show('error', 'Data not found');
  return;
}

const userData = {
  firstName: data.firstName,  // No fallbacks!
  kycStatus: data.kycStatus,
  balance: data.balance
};
```

**RULE:** If data doesn't exist, show an error or redirect. NEVER silently use default values.

---

### 2. **NO localStorage FOR BUSINESS DATA**
❌ **FORBIDDEN:**
```typescript
// WRONG - Storing business data in localStorage
localStorage.setItem('kycStatus', 'verified');
localStorage.setItem('agent_profile_data', JSON.stringify(data));
```

✅ **CORRECT:**
```typescript
// RIGHT - Only use localStorage for UI state
localStorage.setItem('theme', 'dark');  // OK - UI preference
localStorage.setItem('hasSeenModal', 'true');  // OK - UI state

// Business data ALWAYS goes to Juno
await setDoc({
  collection: 'agents',
  doc: { key: principalId, data: agentData }
});
```

**RULE:** localStorage is ONLY for UI preferences and state. ALL business data goes to Juno backend.

---

### 3. **REUSE COMPONENTS - DON'T REINVENT**
❌ **FORBIDDEN:**
```typescript
// WRONG - Duplicating KYC upload logic
<input type="file" onchange={handleIdUpload} />
<input type="file" onchange={handleAddressUpload} />
<input type="file" onchange={handleSelfieUpload} />
// ... 50 lines of upload logic
```

✅ **CORRECT:**
```typescript
// RIGHT - Reuse existing component
import KYCModal from '$lib/components/shared/KYCModal.svelte';

<KYCModal
  isOpen={showKYCModal}
  onClose={() => showKYCModal = false}
  onSubmit={handleKYCSubmit}
/>
```

**RULE:** Before writing ANY component, check if it already exists. Reuse > Reinvent.

---

### 4. **ENCAPSULATION - ONE RESPONSIBILITY PER COMPONENT**
❌ **FORBIDDEN:**
```typescript
// WRONG - God component doing everything
function AgentDashboard() {
  // 500 lines of mixed concerns
  const handleDeposit = () => { /* ... */ };
  const handleWithdraw = () => { /* ... */ };
  const handleKYC = () => { /* ... */ };
  const handleSettings = () => { /* ... */ };
  // ... everything in one file
}
```

✅ **CORRECT:**
```typescript
// RIGHT - Separated concerns
<AgentBalanceCard balance={balance} />
<AgentTransactionHistory transactions={txs} />
<AgentKYCBanner kycStatus={status} />
<AgentSettingsButton />
```

**RULE:** Each component should do ONE thing well. Split large components into smaller, focused ones.

---

### 5. **CONSISTENT DATA FLOW**
```
┌─────────────┐
│   Juno DB   │  ← Single source of truth
└──────┬──────┘
       │
       ▼
┌─────────────┐
│  Component  │  ← Loads from Juno
└──────┬──────┘
       │
       ▼
┌─────────────┐
│   Display   │  ← Shows exact data
└─────────────┘
```

**RULE:** 
- Data flows: Juno → Component → UI
- NO intermediate transformations with fallbacks
- NO localStorage as data source
- NO hardcoded defaults

---

## 📋 SPECIFIC PATTERNS

### Loading Data from Juno
```typescript
async function loadData() {
  const principalId = $principalId;
  if (!principalId) {
    console.log('No principal - redirecting');
    goto('/onboarding');
    return;
  }

  const doc = await getDoc({
    collection: 'users',
    key: principalId
  });

  if (!doc) {
    console.error('❌ DATA ERROR: Document not found');
    toast.show('error', 'Profile not found');
    goto('/onboarding');
    return;
  }

  // Use EXACT data - no fallbacks
  userData = doc.data as UserData;
}
```

### Saving Data to Juno
```typescript
async function saveData() {
  try {
    await setDoc({
      collection: 'users',
      doc: {
        key: $principalId,
        data: {
          ...userData,
          updatedAt: new Date().toISOString()
        }
      }
    });
    toast.show('success', 'Saved successfully!');
  } catch (error) {
    console.error('❌ SAVE ERROR:', error);
    toast.show('error', 'Failed to save');
  }
}
```

### Component Reuse Pattern
```typescript
// 1. Check if component exists
// 2. If yes, import and use it
import ExistingComponent from '$lib/components/shared/ExistingComponent.svelte';

// 3. If no, create NEW shared component
// 4. Put in $lib/components/shared/ for reuse
```

---

## 🚫 ANTI-PATTERNS TO AVOID

### ❌ Silent Fallbacks
```typescript
// WRONG
const name = user?.name || 'Anonymous';
const status = data?.status || 'pending';
```

### ❌ Hardcoded Business Logic
```typescript
// WRONG
if (kycStatus === 'not_started') {
  kycStatus = 'pending';  // Don't change data!
}
```

### ❌ Duplicate Components
```typescript
// WRONG - Creating UserKYCModal AND AgentKYCModal
// RIGHT - One KYCModal with userType prop
```

### ❌ Mixed Concerns
```typescript
// WRONG - UI + Business Logic + Data Access in one function
function handleSubmit() {
  // validation
  // API call
  // UI update
  // navigation
  // all in one place
}
```

---

## ✅ BEST PRACTICES

### 1. **Error Handling**
```typescript
try {
  await operation();
  toast.show('success', 'Success message');
} catch (error: any) {
  console.error('❌ OPERATION ERROR:', error);
  console.error('Details:', {
    message: error.message,
    stack: error.stack,
    principalId: $principalId
  });
  toast.show('error', 'User-friendly error message');
}
```

### 2. **Component Props**
```typescript
// Use TypeScript interfaces
interface Props {
  data: UserData;  // Required
  onSubmit: (data: UserData) => void;  // Required
  isLoading?: boolean;  // Optional
}

let { data, onSubmit, isLoading = false }: Props = $props();
```

### 3. **State Management**
```typescript
// Use Svelte stores for global state
import { principalId } from '$lib/stores/auth';
import { demoMode } from '$lib/stores/demoMode';

// Use local state for component-specific
let isOpen = $state(false);
let formData = $state<FormData>({});
```

### 4. **File Organization**
```
src/
├── lib/
│   ├── components/
│   │   ├── shared/          ← Reusable components
│   │   ├── agent/           ← Agent-specific
│   │   └── user/            ← User-specific
│   ├── stores/              ← Global state
│   ├── services/            ← Business logic
│   └── types/               ← TypeScript types
└── routes/
    ├── agents/              ← Agent pages
    └── users/               ← User pages
```

---

## 🎯 CHECKLIST BEFORE COMMITTING

- [ ] No hardcoded fallback values (`||`, `??`, default params)
- [ ] No localStorage for business data
- [ ] Checked if component already exists
- [ ] Each component has single responsibility
- [ ] Error handling with console.error + toast
- [ ] TypeScript interfaces for props
- [ ] Data flows from Juno → Component → UI
- [ ] No duplicate code
- [ ] Proper encapsulation

---

## 🔍 CODE REVIEW QUESTIONS

1. **Does this data come from Juno?** If not, why?
2. **Does this component already exist?** Check before creating.
3. **What happens if the data is null?** Show error, don't fallback.
4. **Is this component doing too much?** Split if > 200 lines.
5. **Can this be reused?** Put in `/shared` if yes.

---

## 📝 COMMIT MESSAGE FORMAT

```
feat(scope): Short description

- Removed hardcoded fallbacks in UserProfile
- Now loads data strictly from Juno
- Shows error toast if data not found
- Redirects to onboarding if no profile

Fixes #123
```

---

## 🚀 REMEMBER

> **"If you're typing the same code twice, you're doing it wrong."**
> 
> **"If data doesn't exist, show an error. Never fake it."**
> 
> **"localStorage is for UI state, Juno is for business data."**
> 
> **"One component, one job. Keep it simple."**

---

**Last Updated:** 2025-11-03
**Maintained By:** AfriTokeni Dev Team
