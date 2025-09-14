import { useNavigate } from "react-router-dom";
import { useToast } from "@/hooks/use-toast";
import useModalTopupTrashed from "@/store/topup/trashed/modal";
import useTopupTrashedStore from "@/store/topup/trashed/trashed";
import { RestoreTopupTrashed } from "@/types/domain/request/topup";

export default function useRestoreTopup() {
  const {
    restoreTopupId,
    isModalVisibleRestore,
    showModalRestore,
    hideModalRestore,
  } = useModalTopupTrashed();

  const {
    restoreTopupTrashed,
    setLoadingRestoreTopupTrashed,
    loadingRestoreTopupTrashed,
    setErrorRestoreTopupTrashed,
  } = useTopupTrashedStore();

  const navigate = useNavigate();
  const { toast } = useToast();

  const handleSubmit = async () => {
    setLoadingRestoreTopupTrashed(true);

    try {
      await new Promise((resolve) => setTimeout(resolve, 1200));

      const req: RestoreTopupTrashed = {
        id: restoreTopupId as number,
        toast: toast,
      };

      const result = await restoreTopupTrashed(req);

      if (result) {
        toast({
          title: "Success",
          description: "Topup berhasil direstore",
          variant: "default",
        });

        navigate(window.location.pathname);
      } else {
        toast({
          title: "Error",
          description: "Gagal menghapus Topup. Silakan coba lagi.",
          variant: "destructive",
        });
      }
    } catch (error: any) {
      const errorMessage =
        error?.message || "Terjadi kesalahan saat menghapus Topup";
      setErrorRestoreTopupTrashed(errorMessage);
      toast({
        title: "Error",
        description: errorMessage,
        variant: "destructive",
      });
    } finally {
      setLoadingRestoreTopupTrashed(false);
      hideModalRestore();
    }
  };

  return {
    restoreTopupId,
    handleSubmit,
    loadingRestoreTopupTrashed,
    isModalVisibleRestore,
    showModalRestore,
    hideModalRestore,
  };
}
