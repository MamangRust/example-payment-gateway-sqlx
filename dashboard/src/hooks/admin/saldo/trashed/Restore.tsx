import { useNavigate } from "react-router-dom";
import { useToast } from "@/hooks/use-toast";
import useModalSaldoTrashed from "@/store/saldo/trashed/modal";
import useSaldoTrashedStore from "@/store/saldo/trashed/trashed";
import { RestoreSaldoTrashed } from "@/types/domain/request";

export default function useRestoreSaldo() {
  const {
    restoreSaldoId,
    isModalVisibleRestore,
    showModalRestore,
    hideModalRestore,
  } = useModalSaldoTrashed();

  const {
    restoreSaldoTrashed,
    setLoadingRestoreSaldoTrashed,
    loadingRestoreSaldoTrashed,
    setErrorRestoreSaldoTrashed,
  } = useSaldoTrashedStore();

  const navigate = useNavigate();
  const { toast } = useToast();

  const handleSubmit = async () => {
    setLoadingRestoreSaldoTrashed(true);

    try {
      await new Promise((resolve) => setTimeout(resolve, 1200));

      const req: RestoreSaldoTrashed = {
        id: restoreSaldoId as number,
        toast: toast,
      };

      const result = await restoreSaldoTrashed(req);

      if (result) {
        toast({
          title: "Success",
          description: "Saldo berhasil direstore",
          variant: "default",
        });

        navigate(window.location.pathname);
      } else {
        toast({
          title: "Error",
          description: "Gagal menghapus Saldo. Silakan coba lagi.",
          variant: "destructive",
        });
      }
    } catch (error: any) {
      const errorMessage =
        error?.message || "Terjadi kesalahan saat menghapus Saldo";
      setErrorRestoreSaldoTrashed(errorMessage);
      toast({
        title: "Error",
        description: errorMessage,
        variant: "destructive",
      });
    } finally {
      setLoadingRestoreSaldoTrashed(false);
      hideModalRestore();
    }
  };

  return {
    restoreSaldoId,
    handleSubmit,
    loadingRestoreSaldoTrashed,
    isModalVisibleRestore,
    showModalRestore,
    hideModalRestore,
  };
}
