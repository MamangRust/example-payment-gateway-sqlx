import {
  DeletePermanentCard,
  FindAllTrashedCard,
  RestoreTrashedCard,
} from "@/types/domain/request/card/trashed";
import { CardTrashed } from "@/types/model";

export interface CardTrashedStore {
  cards: CardTrashed[] | null;

  pagination: {
    currentPage: number;
    page_size: number;
    totalItems: number;
    totalPages: number;
  };

  loadingGetCardsTrashed: boolean | null;
  loadingRestoreCardTrashed: boolean | null;
  loadingDeletePermanentCardTrashed: boolean | null;
  loadingRestoreAllCardTrashed: boolean | null;
  loadingDeletePermanentAllCardTrashed: boolean | null;

  errorGetCardsTrashed: string | null;
  errorRestoreCardTrashed: string | null;
  errorDeletePermanentCardTrashed: string | null;
  errorRestoreAllCardTrashed: string | null;
  errorDeletePermanentAllCardTrashed: string | null;

  setLoadingGetCardsTrashed: (value: boolean) => void;
  setLoadingRestoreCardTrashed: (value: boolean) => void;
  setLoadingDeletePermanentCardTrashed: (value: boolean) => void;
  setLoadingRestoreAllCardTrashed: (value: boolean) => void;
  setLoadingDeletePermanentAllCardTrashed: (value: boolean) => void;

  setErrorGetCardsTrashed: (value: string) => void;
  setErrorRestoreCardTrashed: (value: string) => void;
  setErrorDeletePermanentCardTrashed: (value: string) => void;
  setErrorRestoreAllCardTrashed: (value: string) => void;
  setErrorDeletePermanentAllCardTrashed: (value: string) => void;

  findAllCardsTrashed: (req: FindAllTrashedCard) => Promise<boolean>;
  restoreCardTrashed: (req: RestoreTrashedCard) => Promise<boolean>;
  deletePermanentCard: (req: DeletePermanentCard) => Promise<boolean>;
  restoreCardAllTrashed: (toast: any) => Promise<boolean>;
  deletePermanentAllCard: (toast: any) => Promise<boolean>;
}
