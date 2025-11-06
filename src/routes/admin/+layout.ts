import type { LayoutLoad } from "./$types";
import { requireRole } from "$lib/auth/roleGuard";

export const load: LayoutLoad = async () => {
  const { user, role } = await requireRole(["admin"]);

  return {
    user,
    role,
  };
};
