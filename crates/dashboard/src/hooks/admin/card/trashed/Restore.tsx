import { useNavigate } from "react-router-dom";
import { useToast } from "@/hooks/use-toast";
import useModalCardTrashed from "@/store/card/trashed/modal";
import useCardTrashedStore from "@/store/card/trashed/trashed";
import { RestoreTrashedCard } from "@/types/domain/request";

export default function useRestoreCard() {
  const {
    restoreCardId,
    isModalVisibleRestore,
    showModalRestore,
    hideModalRestore,
  } = useModalCardTrashed();

  const {
    restoreCardTrashed,
    setLoadingRestoreCardTrashed,
    loadingRestoreCardTrashed,
    setErrorRestoreCardTrashed,
  } = useCardTrashedStore();

  const navigate = useNavigate();
  const { toast } = useToast();

  const handleSubmit = async () => {
    setLoadingRestoreCardTrashed(true);

    try {
      await new Promise((resolve) => setTimeout(resolve, 1200));

      const req: RestoreTrashedCard = {
        id: restoreCardId as number,
        toast: toast,
      };

      const result = await restoreCardTrashed(req);

      if (result) {
        toast({
          title: "Success",
          description: "Card berhasil direstore",
          variant: "default",
        });

        navigate(window.location.pathname);
      } else {
        toast({
          title: "Error",
          description: "Gagal menghapus card. Silakan coba lagi.",
          variant: "destructive",
        });
      }
    } catch (error: any) {
      const errorMessage =
        error?.message || "Terjadi kesalahan saat menghapus card";
      setErrorRestoreCardTrashed(errorMessage);
      toast({
        title: "Error",
        description: errorMessage,
        variant: "destructive",
      });
    } finally {
      setLoadingRestoreCardTrashed(false);
      hideModalRestore();
    }
  };

  return {
    restoreCardId,
    handleSubmit,
    loadingRestoreCardTrashed,
    isModalVisibleRestore,
    showModalRestore,
    hideModalRestore,
  };
}
