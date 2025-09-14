import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import {
  CreateTransferFormValues,
  createTransferRequestSchema,
} from "@/schemas";
import { useToast } from "@/hooks/use-toast";
import useCardStore from "@/store/card/card";
import { zodResolver } from "@hookform/resolvers/zod";
import { forwardRef, useCallback, useEffect, useState } from "react";
import { useForm, Controller } from "react-hook-form";
import AsyncSelect from "react-select/async";
import { TransferCreateFormProps } from "@/types/form";
import { debounce } from "@/helpers/debounce";
import { FindAllCard } from "@/types/domain/request";

interface CardOption {
  value: string;
  label: string;
}

const CreateTransferForm = forwardRef<HTMLFormElement, TransferCreateFormProps>(
  ({ onSubmit }, ref) => {
    const {
      control,
      register,
      handleSubmit,
      formState: { errors },
    } = useForm<CreateTransferFormValues>({
      resolver: zodResolver(createTransferRequestSchema),
      defaultValues: {
        transfer_from: "",
        transfer_to: "",
        transfer_amount: 0,
      },
    });

    const { toast } = useToast();

    const { cards, loadingGetCards, setLoadingGetCards, findAllCards } =
      useCardStore();

    const [transferToOptions, setTransferToOptions] = useState<CardOption[]>(
      [],
    );
    const [transferFromOptions, setTransferFromOptions] = useState<
      CardOption[]
    >([]);

    useEffect(() => {
      const fetchInitialCards = async () => {
        setLoadingGetCards(true);
        try {
          const searchReq: FindAllCard = {
            search: "",
            page: 1,
            page_size: 10,
            toast: toast,
          };
          await findAllCards(searchReq);

          if (cards) {
            const options = cards.map((card) => ({
              value: card.card_number,
              label: card.card_number,
            }));
            setTransferToOptions(options);
            setTransferFromOptions(options);
          }
        } catch (error) {
          console.error("Failed to fetch initial cards:", error);
        } finally {
          setLoadingGetCards(false);
        }
      };

      fetchInitialCards();
    }, []);

    const loadOptions = useCallback(
      debounce(
        async (
          inputValue: string,
          callback: (options: CardOption[]) => void,
        ) => {
          setLoadingGetCards(true);
          try {
            const searchReq: FindAllCard = {
              search: inputValue,
              page: 1,
              page_size: 10,
              toast: toast,
            };
            await findAllCards(searchReq);

            if (cards) {
              const filteredOptions = cards
                .filter((card) =>
                  card.card_number
                    .toLowerCase()
                    .includes(inputValue.toLowerCase()),
                )
                .map((card) => ({
                  value: card.card_number,
                  label: card.card_number,
                }));

              callback(filteredOptions);
            }
          } catch (error) {
            console.error("Failed to fetch cards:", error);
          } finally {
            setLoadingGetCards(false);
          }
        },
        500,
      ),
      [cards, findAllCards, toast],
    );

    return (
      <form ref={ref} onSubmit={handleSubmit(onSubmit)} className="space-y-4">
        <div>
          <Label htmlFor="transfer_from" className="text-sm font-medium">
            Transfer From
          </Label>
          <Controller
            name="transfer_from"
            control={control}
            render={({ field }) => (
              <AsyncSelect
                {...field}
                loadOptions={loadOptions}
                defaultOptions={transferFromOptions}
                isLoading={loadingGetCards}
                placeholder="Select a card to transfer from"
                className="mt-1"
                styles={{
                  control: (base) => ({
                    ...base,
                    borderColor: errors.transfer_from
                      ? "#ef4444"
                      : base.borderColor,
                    borderRadius: "0.375rem",
                    padding: "0.25rem",
                    boxShadow: "none",
                    "&:hover": {
                      borderColor: "#3b82f6",
                    },
                  }),
                  option: (base, { isFocused, isSelected }) => ({
                    ...base,
                    backgroundColor: isSelected
                      ? "#3b82f6"
                      : isFocused
                        ? "#bfdbfe"
                        : "white",
                    color: isSelected ? "white" : "black",
                    "&:hover": {
                      backgroundColor: "#bfdbfe",
                    },
                  }),
                  menu: (base) => ({
                    ...base,
                    borderRadius: "0.375rem",
                    boxShadow: "0 4px 6px -1px rgb(0 0 0 / 0.1)",
                  }),
                }}
                value={transferFromOptions.find(
                  (option) => option.value === field.value,
                )}
                onChange={(selectedOption) => {
                  field.onChange(selectedOption ? selectedOption.value : null);
                }}
              />
            )}
          />
          {errors.transfer_from && (
            <p className="text-red-500 text-sm mt-1">
              {errors.transfer_from.message}
            </p>
          )}
        </div>
        <div>
          <Label htmlFor="transfer_to" className="text-sm font-medium">
            Transfer To
          </Label>
          <Controller
            name="transfer_to"
            control={control}
            render={({ field }) => (
              <AsyncSelect
                {...field}
                loadOptions={loadOptions}
                defaultOptions={transferToOptions}
                isLoading={loadingGetCards}
                placeholder="Select a card to transfer to"
                className="mt-1"
                styles={{
                  control: (base) => ({
                    ...base,
                    borderColor: errors.transfer_to
                      ? "#ef4444"
                      : base.borderColor,
                    borderRadius: "0.375rem",
                    padding: "0.25rem",
                    boxShadow: "none",
                    "&:hover": {
                      borderColor: "#3b82f6",
                    },
                  }),
                  option: (base, { isFocused, isSelected }) => ({
                    ...base,
                    backgroundColor: isSelected
                      ? "#3b82f6"
                      : isFocused
                        ? "#bfdbfe"
                        : "white",
                    color: isSelected ? "white" : "black",
                    "&:hover": {
                      backgroundColor: "#bfdbfe",
                    },
                  }),
                  menu: (base) => ({
                    ...base,
                    borderRadius: "0.375rem",
                    boxShadow: "0 4px 6px -1px rgb(0 0 0 / 0.1)",
                  }),
                }}
                value={transferToOptions.find(
                  (option) => option.value === field.value,
                )}
                onChange={(selectedOption) => {
                  field.onChange(selectedOption ? selectedOption.value : null);
                }}
              />
            )}
          />
          {errors.transfer_to && (
            <p className="text-red-500 text-sm mt-1">
              {errors.transfer_to.message}
            </p>
          )}
        </div>
        <div>
          <Label htmlFor="transfer_amount" className="text-sm font-medium">
            Transfer Amount
          </Label>
          <Input
            id="transfer_amount"
            type="number"
            placeholder="Enter transfer amount"
            className="mt-1"
            {...register("transfer_amount", { valueAsNumber: true })}
          />
          {errors.transfer_amount && (
            <p className="text-red-500 text-sm mt-1">
              {errors.transfer_amount.message}
            </p>
          )}
        </div>
      </form>
    );
  },
);

CreateTransferForm.displayName = "CreateTransferForm";

export default CreateTransferForm;
