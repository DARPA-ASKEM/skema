import os
import requests
from pathlib import Path
import urllib.request
from skema.rest.proxies import SKEMA_MATHJAX_ADDRESS
from skema.img2mml.translate import convert_to_torch_tensor, render_mml
from skema.img2mml.models.image2mml_xfmer import Image2MathML_Xfmer
import torch
from typing import Tuple, List, Any, Dict
from logging import info
from skema.img2mml.translate import define_model
import json
from PIL import Image
from io import BytesIO

from huggingface_hub import hf_hub_download

def retrieve_model(model_path=None) -> str:
    """
    Retrieve the img2mml model from the specified path or download it if not found.

    Args:
        model_path (str, optional): Path to the img2mml model file. Defaults to None.

    Returns:
        str: Path to the loaded model file.
    """
    cwd = Path(__file__).parents[0]
    REPO_NAME = "lum-ai/img2mml"
    MODEL_NAME = "cnn_xfmer_arxiv_im2mml_with_fonts_boldface_best.pt"
    # If the model path is none or doesn't exist, the default model will be downloaded from server.
    if model_path is None or not os.path.exists(model_path):
        model_path = cwd / "trained_models" / MODEL_NAME

        # Check if the model file already exists
        if not os.path.exists(model_path):
            # If the file doesn't exist, download it from the specified URL
            print(f"Downloading the model checkpoint from HuggingFace...")
            hf_hub_download(repo_id=REPO_NAME, filename=MODEL_NAME, local_dir=model_path.parent, local_dir_use_symlinks=False)
        
    return str(model_path)


def check_gpu_availability() -> torch.device:
    """
    Check if GPU is available and return the appropriate device.

    Returns:
        torch.device: The device (GPU or CPU) to be used for computation.
    """
    if not torch.cuda.is_available():
        print("CUDA is not available, falling back to using the CPU.")
        device = torch.device("cpu")
    else:
        device = torch.device("cuda")

    return device


def load_model(
    model_path: str,
    config: dict,
    vocab: List[str],
    device: torch.device = torch.device("cpu"),
) -> Image2MathML_Xfmer:
    """
    Load the model's state dictionary from a file.

    Args:
        model_path: The path to the model state dictionary file.
        config: The configuration setting.
        vocab: The vocabulary dictionary of the img2mml model.
        device: The device (GPU or CPU) to be used for computation.

    Returns:
        The model with loaded state dictionary.

    Raises:
        FileNotFoundError: If the model state dictionary file does not exist.
        RuntimeError: If there is an error during loading the state dictionary.

    Note:
        If `clean_state_dict` is True, the function removes the "module." prefix from the state_dict keys
        if present.

        If CUDA is not available, the function falls back to using the CPU for loading the state dictionary.
    """

    model: Image2MathML_Xfmer = define_model(config, vocab, device).to(device)
    cwd = Path(__file__).parents[0]
    if model_path is None:
        model_path = (
            cwd / "trained_models" / "arxiv_im2mml_with_fonts_with_boldface_best.pt"
        )
    try:
        if not torch.cuda.is_available():
            info("CUDA is not available, falling back to using the CPU.")

        new_model = dict()
        for key, value in torch.load(model_path, map_location=device).items():
            new_model[key[7:]] = value
            model.load_state_dict(new_model, strict=False)

    except FileNotFoundError:
        raise FileNotFoundError(f"Model state dictionary file not found: {model_path}")
    except Exception as e:
        raise RuntimeError(
            f"Error loading state dictionary from file: {model_path}\n{e}"
        )

    return model


def load_vocab(vocab_path: str = None) -> Tuple[List[str], dict, dict]:
    """
    Load vocabulary from a list and create dictionaries for both forward and backward mapping.

    Args:
        vocab (Optional[str, Path]): The vocabulary path.

    Returns:
        Tuple[List[str], dict, dict]: A tuple containing two dictionaries:
            - vocab (List[str]): A complete dictionary.
            - vocab_itos (dict): A dictionary mapping index to token.
            - vocab_stoi (dict): A dictionary mapping token to index.
    """
    cwd = Path(__file__).parents[0]
    if vocab_path is None:
        vocab_path = (
            cwd / "trained_models" / "arxiv_im2mml_with_fonts_with_boldface_vocab.txt"
        )

    # read vocab.txt
    with open(vocab_path) as f:
        vocab = f.readlines()

    vocab_itos = dict()
    vocab_stoi = dict()

    for v in vocab:
        k, v = v.split()
        vocab_itos[v.strip()] = k.strip()
        vocab_stoi[k.strip()] = v.strip()

    return vocab, vocab_itos, vocab_stoi


class Image2MathML:
    def __init__(self, config_path: str, vocab_path: str, model_path: str) -> None:
        self.config = self.load_config(config_path)
        self.vocab, self.vocab_itos, self.vocab_stoi = self.load_vocab(vocab_path)
        self.device = self.check_gpu_availability()
        self.model = self.load_model(model_path)

    def load_config(self, config_path: str) -> Dict[str, Any]:
        with open(config_path, "r") as cfg:
            config = json.load(cfg)
        return config

    def load_vocab(self, vocab_path: str) -> Tuple[Any, Dict[str, Any], Dict[str, Any]]:
        # Load the image2mathml vocabulary
        vocab, vocab_itos, vocab_stoi = load_vocab(vocab_path=vocab_path)
        return vocab, vocab_itos, vocab_stoi

    def check_gpu_availability(self) -> torch.device:
        # Check GPU availability
        if torch.cuda.is_available():
            device = torch.device("cuda")
        else:
            device = torch.device("cpu")
        return device

    def load_model(self, model_path: str) -> Image2MathML_Xfmer:
        # Load the image2mathml model
        MODEL_PATH = retrieve_model(model_path=model_path)
        img2mml_model: Image2MathML_Xfmer = load_model(
            model_path=MODEL_PATH,
            config=self.config,
            vocab=self.vocab,
            device=self.device,
        )
        return img2mml_model


def replace_transparent_background(image_bytes: bytes) -> bytes:
    """
    Replace transparent background with white if the image has transparency.

    Args:
        image_bytes (bytes): Bytes of the input image.

    Returns:
        bytes: Bytes of the processed image with replaced background.
    """
    # Open the image using PIL
    image = Image.open(BytesIO(image_bytes))

    # Check if the image has an alpha (transparency) channel
    if image.mode in ("RGBA", "LA") and image.getchannel("A"):
        # Create a new image with white background
        new_image = Image.new("RGB", image.size, (255, 255, 255))
        new_image.paste(
            image, mask=image.split()[3]
        )  # Paste the original image on the new image with alpha mask
        # Save the new image to bytes
        output_bytes = BytesIO()
        new_image.save(output_bytes, format="PNG")
        return output_bytes.getvalue()
    else:
        # If the image does not have transparency, return the original image data
        return image_bytes


def get_mathml_from_bytes(
    data: bytes,
    image2mathml_db: Image2MathML,
) -> str:
    """
    Convert an image in bytes format to MathML representation using the provided model.

    Args:
        data (bytes): The image data in bytes format.
        model (Image2MathML_Xfmer): The pre-trained image-to-MathML model.
        config (Dict): Configuration dictionary for rendering MathML.
        vocab_itos (Dict): Dictionary mapping index to token for vocabulary.
        vocab_stoi (Dict): Dictionary mapping token to index for vocabulary.
        device (torch.device): CPU or GPU.

    Returns:
        str: The MathML representation of the input image.
    """
    # replace transparent background with white if the image has transparency
    data = replace_transparent_background(data)
    # convert png image to tensor
    imagetensor = convert_to_torch_tensor(data, image2mathml_db.config)

    # change the shape of tensor from (C_in, H, W)
    # to (1, C_in, H, w) [batch =1]
    imagetensor = imagetensor.unsqueeze(0)

    return render_mml(
        image2mathml_db.model,
        image2mathml_db.vocab_itos,
        image2mathml_db.vocab_stoi,
        imagetensor,
        image2mathml_db.device,
    )


def get_mathml_from_file(filepath) -> str:
    """Read an equation image file and convert it to MathML"""

    with open(filepath, "rb") as f:
        data = f.read()

    return get_mathml_from_bytes(data)


def get_mathml_from_latex(eqn: str) -> str:
    """Read a LaTeX equation string and convert it to presentation MathML"""

    # Define the webservice address from the MathJAX service
    webservice = SKEMA_MATHJAX_ADDRESS
    print(f"Connecting to {webservice}")

    # Translate and save each LaTeX string using the NodeJS service for MathJax
    res = requests.post(
        f"{webservice}/tex2mml",
        headers={"Content-type": "application/json"},
        json={"tex_src": eqn},
    )
    if res.status_code == 200:
        return res.text
    else:
        try:
            res.raise_for_status()
        except requests.HTTPError as e:
            return f"HTTP error occurred: {e}"
        except requests.ConnectionError as e:
            return f"Connection error occurred: {e}"
        except requests.Timeout as e:
            return f"Timeout error occurred: {e}"
        except requests.RequestException as e:
            return f"An error occurred: {e}"
        finally:
            return "Conversion Failed."


