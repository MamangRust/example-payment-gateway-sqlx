import { z } from "zod";

export const registerSchema = z
  .object({
    firstname: z
      .string()
      .min(1, "First name is required")
      .max(50, "First name must be less than 50 characters"),

    lastname: z
      .string()
      .min(1, "Last name is required")
      .max(50, "Last name must be less than 50 characters"),

    email: z
      .string()
      .email("Invalid email address")
      .min(1, "Email is required"),

    password: z
      .string()
      .min(8, "Password must be at least 8 characters")
      .regex(/[A-Z]/, "Password must contain at least one uppercase letter")
      .regex(/[0-9]/, "Password must contain at least one number")
      .regex(
        /[!@#$%^&*]/,
        "Password must contain at least one special character",
      ),

    confirm_password: z.string().min(1, "Confirm password is required"),
  })
  .refine((data) => data.password === data.confirm_password, {
    message: "Passwords do not match",
    path: ["confirm_password"],
  });

export type RegisterFormValues = z.infer<typeof registerSchema>;
