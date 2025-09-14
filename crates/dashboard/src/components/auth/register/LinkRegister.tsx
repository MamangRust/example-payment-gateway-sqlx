import { Link } from "react-router-dom";


export default function LinkRegister() {
  return (
    <>
      <div className="text-center">
        <Link
          to="/forgot-password"
          className="text-sm text-muted-foreground hover:underline"
        >
          Forgot your password?
        </Link>
      </div>

      <p className="px-8 text-center text-sm text-muted-foreground">
        By clicking continue, you agree to our{' '}
        <Link
          to="/terms"
          className="underline underline-offset-4 hover:text-primary"
        >
          Terms of Service
        </Link>{' '}
        and{' '}
        <Link
          to="/privacy"
          className="underline underline-offset-4 hover:text-primary"
        >
          Privacy Policy
        </Link>
        .
      </p>

    </>
  )
}
