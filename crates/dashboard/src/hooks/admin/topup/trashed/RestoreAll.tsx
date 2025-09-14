import { useNavigate } from "react-router-dom";
import { useToast } from "@/hooks/use-toast";
import useModalTopupTrashed from "@/store/topup/trashed/modal";
import useTopupTrashedStore from "@/store/topup/trashed/trashed";

export default function useRestoreAllTopup() {
  const { isModalVisibleRestoreAll, showModalRestoreAll, hideModalRestoreAll } =
    useModalTopupTrashed();

  const {
    restoreTopupAllTrashed,
    setLoadingRestoreAllTopupTrashed,
    loadingRestoreAllTopupTrashed,
    setErrorRestoreAllTopupTrashed,
  } = useTopupTrashedStore();

  const navigate = useNavigate();
  const { toast } = useToast();

  const handleSubmit = async () => {
    setLoadingRestoreAllTopupTrashed(true);

    try {
      await new Promise((resolve) => setTimeout(resolve, 1200));

      const result = await restoreTopupAllTrashed(toast);

      if (result) {
        toast({
          title: "Success",
          description: "Topup berhasil dihapus",
          variant: "default",
        });

        navigate(window.location.pathname);
      } else {
        toast({
          title: "Error",
          description: "Gagal restore Topup. Silakan coba lagi.",
          variant: "destructive",
        });
      }
    } catch (error: any) {
      const errorMessage =
        error?.message || "Terjadi kesalahan saat restore Topup";
      setErrorRestoreAllTopupTrashed(errorMessage);
      toast({
        title: "Error",
        description: errorMessage,
        variant: "destructive",
      });
    } finally {
      setLoadingRestoreAllTopupTrashed(false);
      hideModalRestoreAll();
    }
  };

  return {
    handleSubmit,
    loadingRestoreAllTopupTrashed,
    isModalVisibleRestoreAll,
    showModalRestoreAll,
    hideModalRestoreAll,
  };
}
