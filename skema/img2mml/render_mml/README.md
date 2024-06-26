# MathML and LaTeX Comparison Visualization

This folder contains the following components:

1. **Mathpix Annotator**: A script to post our images to [Mathpix](https://docs.mathpix.com/#introduction) and retrieve Mathpix's OCR annotations for the images.

2. **OCR Hub**: A landing page that contains links to the Mathpix OCR Tweak-er and the OCR Verify web apps.

In general, you can skip to the OCR Hub usage section since the Mathpix Annotator doesn't need to be run unless we want to regenerate Mathpix's OCR results.

## Mathpix Annotator

The Mathpix Annotator is a script used to post our images to Mathpix and obtain Mathpix's OCR annotations for those images. The script has already been executed once, and the results from Mathpix have been stored.

**NOTE:** Do not re-run the Mathpix Annotator unless we specifically need Mathpix to annotate our images again. The code that queries Mathpix's API has been commented out, and it should only be uncommented and run if necessary.

### Usage

1. Create a file called `config.py` in the `mathpix_annotator/` directory and save your `MATHPIX_API_KEY` there.
2. Run `main.ipynb`. The code will reuse the results stored from the previous run instead of re-querying Mathpix. If you want to regenerate the results from the API, uncomment the relevant sections **only if needed**.

### Generated Files

1. `mathpix_full_results.json`: Contains the raw results from Mathpix.
2. `mathpix_errors.json`: Contains results with errors from Mathpix.
3. `mathpix_ocr_tweaker_data.json`: Contains trimmed Mathpix results sorted in increasing order of confidence. This data is meant to be used in the Mathpix OCR Tweak-er.

### Other Files

1. `main.ipynb`: This file serves as the driver code for the Mathpix Annotator script.
2. `batchRequestHandler`: This is a helper class used for making Mathpix batch requests.
3. `image_ids.json`: This JSON file holds the IDs for the images being posted.

## OCR Hub

The OCR Hub is a landing page that provides links to the Mathpix OCR Tweak-er and the OCR Verify pages. The functions of these pages are as follows:

- **Mathpix OCR Tweak-er**: A web app used to individually visualize the Mathpix annotations and make any manual corrections if required. Both the LaTeX code and the MathML code generated by Mathpix can be manually corrected to match the original image here.

- **OCR Verify**: A web app used to visualize the OCR results of our own model and compare the output MathML to the original image. (Currently using placeholder data)

### Usage

```bash
cd ocr_hub
pip install -r requirements.txt # Installs FastAPI
uvicorn main:app # Runs the web app
```

The app should now be running. You can view it on [localhost:8000](http://127.0.0.1:8000/).

### Files

- `main.py`: This file contains the backend API.
- `static/`: This directory contains the HTML, JS, and CSS files for the sites.
- `data/`: This directory contains the data stores for each web app.
