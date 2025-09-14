import { z } from "zod";

export const updateRoleRequestSchema = z
  .object({
    name: z
      .string()
      .min(1, "Role name is required")
      .max(15, "Role name must be less than 15 characters"),

  })

export type UpdateRoleFormValues = z.infer<typeof updateRoleRequestSchema>;
