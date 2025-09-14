import { useNavigate } from "react-router-dom";
import { useToast } from "@/hooks/use-toast";
import useModalSaldoTrashed from "@/store/saldo/trashed/modal";
import useSaldoTrashedStore from "@/store/saldo/trashed/trashed";

export default function useRestoreAllSaldo() {
  const { isModalVisibleRestoreAll, showModalRestoreAll, hideModalRestoreAll } =
    useModalSaldoTrashed();

  const {
    restoreSaldoAllTrashed,
    setLoadingRestoreAllSaldoTrashed,
    loadingRestoreAllSaldoTrashed,
    setErrorRestoreAllSaldoTrashed,
  } = useSaldoTrashedStore();

  const navigate = useNavigate();
  const { toast } = useToast();

  const handleSubmit = async () => {
    setLoadingRestoreAllSaldoTrashed(true);

    try {
      await new Promise((resolve) => setTimeout(resolve, 1200));

      const result = await restoreSaldoAllTrashed(toast);

      if (result) {
        toast({
          title: "Success",
          description: "Saldo berhasil dihapus",
          variant: "default",
        });

        navigate(window.location.pathname);
      } else {
        toast({
          title: "Error",
          description: "Gagal restore Saldo. Silakan coba lagi.",
          variant: "destructive",
        });
      }
    } catch (error: any) {
      const errorMessage =
        error?.message || "Terjadi kesalahan saat restore Saldo";
      setErrorRestoreAllSaldoTrashed(errorMessage);
      toast({
        title: "Error",
        description: errorMessage,
        variant: "destructive",
      });
    } finally {
      setLoadingRestoreAllSaldoTrashed(false);
      hideModalRestoreAll();
    }
  };

  return {
    handleSubmit,
    loadingRestoreAllSaldoTrashed,
    isModalVisibleRestoreAll,
    showModalRestoreAll,
    hideModalRestoreAll,
  };
}
