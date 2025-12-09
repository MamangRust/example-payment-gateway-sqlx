import LoginForm from "@/components/auth/login/LoginForm";
import LinkLogin from "@/components/auth/login/LinkLogin";
import useLogin from "@/hooks/auth/Login";

export default function LoginPage() {
  const { onFinish, loadingLogin } = useLogin();

  return (
    <div className="flex w-full lg:w-1/2 items-center justify-center p-8 bg-background">
      <div className="mx-auto w-full max-w-sm space-y-6">
        <div className="flex flex-col space-y-2 text-center">
          <h1 className="text-2xl font-semibold tracking-tight">
            Welcome back
          </h1>
          <p className="text-sm text-muted-foreground">
            Enter your email to sign in to your account
          </p>
        </div>
        <LoginForm handleSubmit={onFinish} loadingLogin={loadingLogin} />
        <LinkLogin />
      </div>
    </div>
  );
}
