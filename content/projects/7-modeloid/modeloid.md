
# Introduction

I made an app to make training computer vision models easier. It is completely open-source and runs locally, on Rust language. Here's what it can do.

# Why Did I Built It

There are pretty well known workbenches that already exist today. But most of them work on cloud spaces, which is not a bad thing. However every single one of they require tokens and payment for space, processing power or statistics. But why pay for space or processing power if you have space locally and you have a sufficient device to train your models on, and why pay for statistics when most of the statistics libraries are already free? I wanted to use an application like this myself, so I started this project.

The application runs on Rust to perform file operations and uses Python libraries for ML operations. This combo makes it really convenient and fast for a competitive usage against the cloud alternatives.

# Installation

You can install the application in the Releases tab in the GitHub repository. You can download the Windows setup file to quickly get started. The application is not yet tested on MacOS or Linux environments but feel free to clone and use the project as you wish.

# Usage

When you first install and start the app, a web browser should open. Before anything you should create a new workspace. A workspace is where you do all your operations using this app. After you create your workspace, there are a couple of things can do.

## Dataset Manager

### Datasets

I am assuming you have a goal for training a model. For that you most likely need a dataset, not one but many. In the Dataset Manager, Datasets section you are able to Create a new dataset if you have images only, edit these datasets and label them, download from a known source, or import an existing dataset from your computer.

If you want to create your own dataset, this app provides an in-built annotation editor to create labels. Or maybe you don't have a dataset and want to download existing ones. For that you should find the dataset you're looking for in Roboflow, Kaggle or HuggingFace and put the link of the project here. You can also do batch download if you want, just put every link in a new line. Before you download though, you need to enter the download source API keys in Settings section. If you need to know where they are, just click on the "Where to find..." links below corresponding API Key section.

When you have one or more datasets, they are displayed in this table section on the right and their properties. You can collapse or expand the table as you wish.

### Explore

The explore section shows you that dataset's statistics when a dataset is selected. You can display the images in that dataset in the Gallery section.

### Health Check

The health check section shows probably issues that is caught by the app. After you run the Health Check on a dataset, if they exist, from corrupt images to duplicates and many more will be alerted to you, and if you want to handle them you can do it right here.

### Classes & Merge

In the classes and merge section allows you to change class names or merge many datasets into one over, again, class names. Simply select the check boxes left to the dataset names on the table to select one than more datasets to get started.

### Split & Balance

The split and balance section is where you do train test splits or if you want to, balance the classes interactively on the Class Distribution graph.

### Convert

The convert section is mainly used to change annotation formats or downgrade dataset types. Let's say you have a COCO JSON dataset and you want convert it to YOLO, you can do it here. Or maybe you need to train a bounding box based model but you have segmentation datasets, you can use the conversion choices here to convert it to whatever you like.

### Augmentation

The data augmentation section can be used to augment the data you have in your workspace. It uses the Albumentations library perform augmentation.

## Training

### Parameters

After you prepared your dataset, you are ready to train your model. However before training you should be preparing a parameter set in the Parameters section. You can use an existing dataset to start adjusting. If you like to, you can adjust the hyperparameters or training parameters in this section.

### Train Model

Let's start training. In the Train Model section, you need to select a dataset to be trained on and a parameter set to start the training. Start training and it will show the progress.

### Performance

After the training is done, you can display the training run statistics in Performance section to see how well your model performs.

### Tune Hyperparameters

The experimental feature Tune Hyperparameters section can be used to tune the parameters by creating a search space, comparing each candidate and coming with a most optimized set.

## Miscellaneous

The History section can show you any operation you did in that workspace by its name and date.

When you go to Settings, there are some things you can check out. Firstly you can change the appearance, the font, or the text size to your liking. Secondly you can adjust system usage in the System section by changing worker threads, download connections, jobs at once or preview size. Below that you can clear cache, this will delete the history as well. In the Python Environment section, normally the training device is built to be on the CPU by default. But if you have a CUDA compatible device you can install the CUDA libraries with install here to speed up the training.
