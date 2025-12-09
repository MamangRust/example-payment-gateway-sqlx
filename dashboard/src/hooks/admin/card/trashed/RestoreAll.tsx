import { useNavigate } from "react-router-dom";
import { useToast } from "@/hooks/use-toast";
import useModalCardTrashed from "@/store/card/trashed/modal";
import useCardTrashedStore from "@/store/card/trashed/trashed";

export default function useRestoreAllCard() {
  const { isModalVisibleRestoreAll, showModalRestoreAll, hideModalRestoreAll } =
    useModalCardTrashed();

  const {
    restoreCardAllTrashed,
    setLoadingRestoreAllCardTrashed,
    loadingRestoreAllCardTrashed,
    setErrorRestoreAllCardTrashed,
  } = useCardTrashedStore();

  const navigate = useNavigate();
  const { toast } = useToast();

  const handleSubmit = async () => {
    setLoadingRestoreAllCardTrashed(true);

    try {
      await new Promise((resolve) => setTimeout(resolve, 1200));

      const result = await restoreCardAllTrashed(toast);

      if (result) {
        toast({
          title: "Success",
          description: "Card berhasil dihapus",
          variant: "default",
        });

        navigate(window.location.pathname);
      } else {
        toast({
          title: "Error",
          description: "Gagal restore card. Silakan coba lagi.",
          variant: "destructive",
        });
      }
    } catch (error: any) {
      const errorMessage =
        error?.message || "Terjadi kesalahan saat restore card";
      setErrorRestoreAllCardTrashed(errorMessage);
      toast({
        title: "Error",
        description: errorMessage,
        variant: "destructive",
      });
    } finally {
      setLoadingRestoreAllCardTrashed(false);
      hideModalRestoreAll();
    }
  };

  return {
    handleSubmit,
    loadingRestoreAllCardTrashed,
    isModalVisibleRestoreAll,
    showModalRestoreAll,
    hideModalRestoreAll,
  };
}
