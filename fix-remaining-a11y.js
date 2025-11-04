#!/usr/bin/env node
/**
 * Fix remaining accessibility warnings
 * - Add IDs to all inputs/ranges and for attributes to labels
 * - Add aria-labels to icon-only buttons
 */

import fs from 'fs';
import path from 'path';
import { fileURLToPath } from 'url';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

function generateId(text) {
  return text
    .replace(/[^\w\s]/g, '')
    .split(/\s+/)
    .filter(w => w)
    .map((word, i) => 
      i === 0 
        ? word.toLowerCase() 
        : word.charAt(0).toUpperCase() + word.slice(1).toLowerCase()
    )
    .join('') || 'input' + Date.now();
}

function fixFile(filePath) {
  console.log(`\n📝 ${path.basename(filePath)}`);
  let content = fs.readFileSync(filePath, 'utf8');
  let changes = 0;
  
  // Fix: <label>Text</label> followed by <input without id
  const lines = content.split('\n');
  const newLines = [];
  
  for (let i = 0; i < lines.length; i++) {
    const line = lines[i];
    
    // Check if this is a label without 'for' attribute
    if (line.includes('<label') && !line.includes('for=') && line.includes('class=')) {
      // Get the label text
      let labelText = '';
      let j = i;
      while (j < lines.length && !lines[j].includes('</label>')) {
        if (lines[j].includes('>') && !lines[j].includes('<label')) {
          labelText += lines[j].split('>').slice(1).join('>');
        }
        j++;
      }
      if (j < lines.length) {
        labelText += lines[j].split('</label>')[0];
      }
      labelText = labelText.replace(/<[^>]+>/g, '').trim();
      
      // Find the next input/range after this label
      let foundInput = false;
      for (let k = i + 1; k < Math.min(i + 10, lines.length); k++) {
        if ((lines[k].includes('<input') || lines[k].includes('type="range"')) && !lines[k].includes('id=')) {
          const id = generateId(labelText);
          
          // Add for= to label
          newLines.push(line.replace('<label', `<label for="${id}"`));
          
          // Process lines until we find the input
          for (let m = i + 1; m < k; m++) {
            newLines.push(lines[m]);
          }
          
          // Add id= to input
          const inputLine = lines[k];
          if (inputLine.includes('<input')) {
            newLines.push(inputLine.replace('<input', `<input\n              id="${id}"`));
          } else {
            newLines.push(inputLine.replace('type="range"', `id="${id}"\n            type="range"`));
          }
          
          i = k;
          foundInput = true;
          changes++;
          break;
        }
      }
      
      if (!foundInput) {
        newLines.push(line);
      }
    }
    // Fix icon-only buttons without aria-label
    else if (line.includes('<button') && !line.includes('aria-label') && i + 1 < lines.length) {
      const nextLine = lines[i + 1];
      if (nextLine.includes('class="h-') && nextLine.includes('w-')) {
        // This is an icon button
        let ariaLabel = 'Toggle';
        if (content.substring(Math.max(0, i - 500), i).includes('Bitcoin')) ariaLabel = 'Toggle Bitcoin';
        else if (content.substring(Math.max(0, i - 500), i).includes('Location')) ariaLabel = 'Toggle location';
        else if (content.substring(Math.max(0, i - 500), i).includes('Notification')) ariaLabel = 'Toggle notifications';
        else if (content.substring(Math.max(0, i - 500), i).includes('SMS')) ariaLabel = 'Toggle SMS';
        else if (content.substring(Math.max(0, i - 500), i).includes('Email')) ariaLabel = 'Toggle email';
        else if (content.substring(Math.max(0, i - 500), i).includes('Security')) ariaLabel = 'Toggle security';
        
        newLines.push(line.replace('<button', `<button aria-label="${ariaLabel}"`));
        changes++;
      } else {
        newLines.push(line);
      }
    }
    else {
      newLines.push(line);
    }
  }
  
  if (changes > 0) {
    fs.writeFileSync(filePath, newLines.join('\n'), 'utf8');
    console.log(`✅ Fixed ${changes} issues`);
  } else {
    console.log(`✓ No issues`);
  }
  
  return changes;
}

const files = [
  'src/lib/components/shared/AgentSettingsComponent.svelte',
  'src/lib/components/shared/CustomerManagementComponent.svelte',
  'src/lib/components/dashboard/OnboardingModal.svelte',
  'src/lib/components/shared/KYCModal.svelte',
  'src/lib/components/landing/SavingsComparisonTable.svelte',
  'src/routes/users/profile/AccountSettings.svelte',
];

console.log('🔧 Fixing Remaining Accessibility Warnings\n');

let total = 0;
files.forEach(file => {
  const fullPath = path.join(__dirname, file);
  if (fs.existsSync(fullPath)) {
    total += fixFile(fullPath);
  }
});

console.log(`\n✨ Fixed ${total} issues`);
console.log('🔍 Run: npm run check');
