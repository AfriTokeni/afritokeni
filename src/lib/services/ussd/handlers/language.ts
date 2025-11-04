/**
 * Language Selection Handler
 * Allows users to change their preferred language
 */

import type { USSDSession } from "../types.js";
import { continueSession } from "../utils/responses.js";
import { TranslationService, type Language } from "../../translations.js";

// In-memory storage for language preferences (in production, this would be in a database)
const languagePreferences = new Map<string, Language>();

/**
 * Normalize phone number format (remove + prefix for consistent storage)
 */
function normalizePhoneNumber(phoneNumber: string): string {
  return phoneNumber.replace(/^\+/, '');
}

/**
 * Save language preference for a phone number
 */
export async function saveLanguagePreference(phoneNumber: string, language: Language): Promise<void> {
  const normalized = normalizePhoneNumber(phoneNumber);
  languagePreferences.set(normalized, language);
  console.log(`💾 Saved language preference for ${normalized}: ${language}`);
  console.log(`📊 Total preferences in map: ${languagePreferences.size}`);
}

/**
 * Get language preference for a phone number
 */
export async function getLanguagePreference(phoneNumber: string): Promise<Language | null> {
  const normalized = normalizePhoneNumber(phoneNumber);
  const language = languagePreferences.get(normalized) || null;
  console.log(`🔍 Retrieved language preference for ${normalized}: ${language}`);
  return language;
}

/**
 * Clear all language preferences (for testing)
 */
export function clearLanguagePreferences(): void {
  languagePreferences.clear();
  console.log('🧹 Cleared all language preferences');
}

/**
 * Handle language selection menu
 */
export async function handleLanguageSelection(
  input: string,
  session: USSDSession,
): Promise<string> {
  const inputParts = input.split("*");
  // For chained input from main menu (e.g., "6*2"), extract the second part
  // Otherwise extract the last part for direct navigation
  const currentInput =
    inputParts.length > 1 && inputParts[0] === "6"
      ? inputParts[1]
      : inputParts[inputParts.length - 1] || "";

  // Show language menu
  if (!currentInput) {
    return continueSession(
      TranslationService.translate("select_language", session.language || "en"),
    );
  }

  // Handle language selection
  switch (currentInput) {
    case "1":
      session.language = "en";
      await saveLanguagePreference(session.phoneNumber, "en");
      return continueSession(
        `${TranslationService.translate("language_set", "en")}\n\n${TranslationService.translate("back_or_menu", "en")}`,
      );

    case "2":
      session.language = "lg";
      await saveLanguagePreference(session.phoneNumber, "lg");
      return continueSession(
        `${TranslationService.translate("language_set", "lg")}\n\n${TranslationService.translate("back_or_menu", "lg")}`,
      );

    case "3":
      session.language = "sw";
      await saveLanguagePreference(session.phoneNumber, "sw");
      return continueSession(
        `${TranslationService.translate("language_set", "sw")}\n\n${TranslationService.translate("back_or_menu", "sw")}`,
      );

    case "0":
      // Go back to main menu - need to import and call it
      session.currentMenu = "main";
      session.step = 0;
      // Return a marker that tells the service to show main menu
      return continueSession("__SHOW_MAIN_MENU__");

    default:
      return continueSession(
        TranslationService.translate(
          "select_language",
          session.language || "en",
        ),
      );
  }
}
