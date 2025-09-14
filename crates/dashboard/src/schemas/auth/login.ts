import { z } from 'zod';

export const loginSchema = z.object({
  email: z.string().email("Email tidak valid"),
  password: z.string().min(6, "Password harus memiliki setidaknya 6 karakter"),
});

export type LoginFormValues = z.infer<typeof loginSchema>;