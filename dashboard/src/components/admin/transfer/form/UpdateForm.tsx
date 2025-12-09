import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import {
  updateTransferRequestSchema,
  UpdateTransferFormValues,
} from "@/schemas";
import { zodResolver } from "@hookform/resolvers/zod";
import { useForm, Controller } from "react-hook-form";
import { forwardRef, useCallback, useEffect, useState } from "react";
import { TransferUpdateFormProps } from "@/types/form";
import AsyncSelect from "react-select/async";
import { useToast } from "@/hooks/use-toast";
import useCardStore from "@/store/card/card";
import { debounce } from "@/helpers/debounce";
import { FindAllCard } from "@/types/domain/request";

interface CardOption {
  value: string;
  label: string;
}

const UpdateTransferForm = forwardRef<HTMLFormElement, TransferUpdateFormProps>(
  ({ onSubmit, defaultValues }, ref) => {
    const {
      control,
      register,
      handleSubmit,
      formState: { errors },
    } = useForm<UpdateTransferFormValues>({
      resolver: zodResolver(updateTransferRequestSchema),
      defaultValues: defaultValues || {
        transfer_from: {
          value: "",
          label: "",
        },
        transfer_to: {
          value: "",
          label: "",
        },
        transfer_amount: 0,
      },
    });

    const { toast } = useToast();

    const {
      cards,
      loadingGetCards,
      setLoadingGetCards,
      findAllCards,
      card,
      findByCardNumber,
    } = useCardStore();

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

            if (defaultValues) {
              const updateDefaultOptions = (defaultValue: any) => {
                if (defaultValue?.value) {
                  const isCardInList = options.some(
                    (opt) => opt.value === defaultValue.value,
                  );
                  if (!isCardInList) {
                    const req = {
                      toast,
                      cardNumber: defaultValue.value,
                    };
                    findByCardNumber(req);
                    if (card) {
                      options.unshift({
                        value: card.card_number,
                        label: card.card_number,
                      });
                    }
                  }
                }
                return options;
              };

              const transferFromOptions = updateDefaultOptions(
                defaultValues.transfer_from,
              );
              const transferToOptions = updateDefaultOptions(
                defaultValues.transfer_to,
              );

              setTransferFromOptions(transferFromOptions);
              setTransferToOptions(transferToOptions);
            } else {
              setTransferFromOptions(options);
              setTransferToOptions(options);
            }
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
      debounce(async (inputValue, callback) => {
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
          callback([]);
        } finally {
          setLoadingGetCards(false);
        }
      }, 500),
      [cards, findAllCards, toast],
    );

    return (
      <form ref={ref} onSubmit={handleSubmit(onSubmit)} className="space-y-4">
        <div>
          <Label htmlFor="transferFrom" className="text-sm font-medium">
            Transfer From
          </Label>
          <Controller
            name="transfer_from"
            control={control}
            render={({ field }) => (
              <AsyncSelect
                key={transferFromOptions.length}
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
                value={
                  transferFromOptions.find(
                    (option) =>
                      option.value ===
                      (typeof field.value === "object"
                        ? field.value?.value
                        : field.value),
                  ) || {
                    value: defaultValues?.transfer_from?.value || "",
                    label:
                      defaultValues?.transfer_from?.label ||
                      "Select a card to transfer from",
                  }
                }
                onChange={(selectedOption) => {
                  field.onChange(
                    selectedOption
                      ? {
                          value: selectedOption.value,
                          label: selectedOption.label,
                        }
                      : null,
                  );
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

        <div className="mt-4">
          <Label htmlFor="transferTo" className="text-sm font-medium">
            Transfer To
          </Label>
          <Controller
            name="transfer_to"
            control={control}
            render={({ field }) => (
              <AsyncSelect
                key={transferToOptions.length}
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
                value={
                  transferToOptions.find(
                    (option) =>
                      option.value ===
                      (typeof field.value === "object"
                        ? field.value?.value
                        : field.value),
                  ) || {
                    value: defaultValues?.transfer_to?.value || "",
                    label:
                      defaultValues?.transfer_to?.label ||
                      "Select a card to transfer to",
                  }
                }
                onChange={(selectedOption) => {
                  field.onChange(
                    selectedOption
                      ? {
                          value: selectedOption.value,
                          label: selectedOption.label,
                        }
                      : null,
                  );
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

UpdateTransferForm.displayName = "UpdateTransferForm";

export default UpdateTransferForm;
